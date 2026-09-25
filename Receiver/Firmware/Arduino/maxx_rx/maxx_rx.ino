// Maxx Steele receiver: nRF905 packets -> OOK envelope on RadioIn.
// Wiring and timing: Receiver/Firmware/Arduino/README.md
//
// ATtiny85 at 3.3 V, internal 8 MHz. Matches maxx_remote on the air.
// PB4 drives a 2N2222 that inverts; the collector is the 5 V RadioIn pin.
// PWR_UP and TRX_CE are tied to 3.3 V. TX_EN is tied to GND.
// Do not connect the nRF905 to the robot's 5 V rail.

#include <avr/io.h>

// 1: 2N2222 inverts, so a high on PB4 means carrier absent at the collector.
// Set to 0 if PB4 drives RadioIn through a non-inverting buffer instead.
static const uint8_t DATA_INVERT = 1;

// ATTinyCore pin numbers. SPI is bit-banged so MOSI stays PB0 (not the USI DO pin).
static const uint8_t PIN_MOSI = PIN_PB0;
static const uint8_t PIN_MISO = PIN_PB1;
static const uint8_t PIN_SCK = PIN_PB2;
static const uint8_t PIN_CSN = PIN_PB3;
static const uint8_t PIN_DATA = PIN_PB4;
static_assert(PIN_MOSI == 0 && PIN_MISO == 1 && PIN_SCK == 2 && PIN_CSN == 3 && PIN_DATA == 4,
              "ATTinyCore ATtiny85 pin map");

// (422.4 + channel/10) * (1 + HFREQ_PLL) MHz. Same channel as the remote.
static const uint16_t RADIO_CHANNEL = 108;
static const uint8_t RADIO_HFREQ_PLL = 0;
static const uint8_t RADIO_ADDR[] = {'M', 'A', 'X', 'X'};
static const uint8_t PAYLOAD_LEN = 2;
static const uint8_t ADDR_LEN = sizeof(RADIO_ADDR);
static_assert(ADDR_LEN == 4, "nRF905 address is 4 bytes");
static_assert(F_CPU == 8000000UL, "build for the 8 MHz internal oscillator");

static const uint8_t RADIO_XOF_16MHZ = 0x18;
static const uint8_t RADIO_CRC16 = 0xC0;

// Measured OOK cell, about 645 baud. See transmitter-architecture.md.
// Trim on the robot; the ROM sample loop is not in the listing.
static const unsigned long BIT_US = 1550;
static const unsigned long FRAME_US = 29000;
static const unsigned long FRAME_Y_US = 21000;
static const unsigned long HOLD_MS = 80;
static const unsigned long RADIO_RETRY_MS = 1000;
// nRF905 product spec: crystal start-up is typically 3 ms after PWR_UP rises.
static const unsigned long RADIO_OSC_START_MS = 5;
// Leave this much gap before the frame deadline so a SPI read cannot eat the next cell.
static const unsigned long SPI_GAP_GUARD_US = 200;

static_assert(11 * BIT_US < FRAME_US, "11-bit burst must finish inside the frame");
static_assert(13 * BIT_US < FRAME_Y_US, "Power/Stop burst must finish inside the frame");

static const uint8_t CMD_W_CONFIG = 0x00;
static const uint8_t CMD_R_CONFIG = 0x10;
static const uint8_t CMD_R_RX_PAYLOAD = 0x24;
static const uint8_t CMD_W_TX_ADDRESS = 0x22;
static const uint8_t CMD_R_TX_ADDRESS = 0x23;
static const uint8_t STATUS_DR = 0x20;

// Repeat / release words from the transmitter reverse-engineering notes.
// Leftmost bit is sent first. Y has no release word.
struct OokWord {
  uint16_t repeat;
  uint16_t release;
  uint8_t nbits;
};

static const OokWord WORDS[] = {
    {0b10101000101, 0b11010000101, 11},  // A  0
    {0b10101000110, 0b11010000110, 11},  // B  1
    {0b11001000111, 0b11110000111, 11},  // C  2
    {0b10101001000, 0b11010001000, 11},  // D  3
    {0b11001001001, 0b11110001001, 11},  // E  4
    {0b11001001010, 0b11110001010, 11},  // F  5
    {0b11101001011, 0b10010001011, 11},  // G  6
    {0b10101010100, 0b11010010100, 11},  // H  7
    {0b11001010101, 0b11110010101, 11},  // I  8
    {0b11001010110, 0b11110010110, 11},  // J  9
    {0b11101010111, 0b10010010111, 11},  // K  LAMP
    {0b11001011000, 0b11110011000, 11},  // L  HOME
    {0b11101011001, 0b10010011001, 11},  // M  NOTE REST
    {0b11101011010, 0b10010011010, 11},  // N  SHIFT OCTAVE
    {0b10001011011, 0b10110011011, 11},  // O  CLEAR
    {0b10101100100, 0b11010100100, 11},  // P  ENTER
    {0b11001100101, 0b11110100101, 11},  // Q  SONG/NOTES
    {0b11001100110, 0b11110100110, 11},  // R  CLOCK/STATUS
    {0b11101100111, 0b10010100111, 11},  // S  SPEECH
    {0b11001101000, 0b11110101000, 11},  // T  MOTION
    {0b11101101001, 0b10010101001, 11},  // U  GAME
    {0b11101101010, 0b10010101010, 11},  // V  PROGRAM
    {0b10001101011, 0b10110101011, 11},  // W  LEARN
    {0b11001110100, 0b11110110100, 11},  // X  EXECUTE
    {0b1110111010100, 0, 13},            // Y  POWER/STOP
};
static const uint8_t WORD_COUNT = sizeof(WORDS) / sizeof(WORDS[0]);
static_assert(WORD_COUNT == 25, "one OOK word per key A-Y");

enum Phase : uint8_t { PHASE_IDLE = 0, PHASE_BURST, PHASE_GAP };

static Phase phase = PHASE_IDLE;
static uint16_t burstWord = 0;
static uint8_t burstBits = 0;
static uint8_t bitIndex = 0;
static unsigned long bitDeadline = 0;
static unsigned long frameDeadline = 0;
static bool frameIsRelease = false;

static char activeKey = 0;
static unsigned long activeRxMs = 0;
static bool stopRequested = false;
static char queuedKey = 0;

static bool radioOk = false;
static bool sawDrLow = true;
static unsigned long radioRetryAt = 0;

static void writeCarrier(bool on) {
  bool high = on;
  if (DATA_INVERT) {
    high = !high;
  }
  digitalWrite(PIN_DATA, high ? HIGH : LOW);
}

static bool wordBit(uint16_t word, uint8_t nbits, uint8_t index) {
  return ((word >> (nbits - 1 - index)) & 1) != 0;
}

static int8_t wordIndex(char key) {
  if (key < 'A' || key > 'Y') {
    return -1;
  }
  return (int8_t)(key - 'A');
}

static void startFrame(uint16_t word, uint8_t nbits, bool release) {
  burstWord = word;
  burstBits = nbits;
  bitIndex = 0;
  frameIsRelease = release;
  writeCarrier(wordBit(word, nbits, 0));
  unsigned long now = micros();
  bitDeadline = now + BIT_US;
  frameDeadline = now + (nbits > 11 ? FRAME_Y_US : FRAME_US);
  phase = PHASE_BURST;
}

static void finishFrame() {
  writeCarrier(false);
  if (frameIsRelease) {
    char next = queuedKey;
    queuedKey = 0;
    activeKey = 0;
    stopRequested = false;
    frameIsRelease = false;
    if (next) {
      activeKey = next;
      activeRxMs = millis();
    }
  }
  phase = PHASE_IDLE;
}

static void onPacket(char key, char edge) {
  if (frameIsRelease) {
    if (edge == 'D') {
      queuedKey = key;
    }
    return;
  }
  if (edge == 'D') {
    if (activeKey == key) {
      activeRxMs = millis();
      stopRequested = false;
      return;
    }
    if (activeKey == 0) {
      activeKey = key;
      activeRxMs = millis();
      stopRequested = false;
      return;
    }
    queuedKey = key;
    stopRequested = true;
    return;
  }
  if (edge != 'U') {
    return;
  }
  if (queuedKey == key) {
    queuedKey = 0;
  }
  if (activeKey == key) {
    stopRequested = true;
  }
}

// Mode 0: clock idle low, nRF905 samples MOSI on the rising edge.
// Port toggles stay well under the 10 MHz SPI limit at 8 MHz.
static uint8_t spiByte(uint8_t out) {
  uint8_t in = 0;
  for (uint8_t bit = 0; bit < 8; bit++) {
    if (out & 0x80) {
      PORTB |= _BV(PB0);
    } else {
      PORTB &= ~_BV(PB0);
    }
    out = (uint8_t)(out << 1);
    PORTB |= _BV(PB2);
    in = (uint8_t)((in << 1) | ((PINB & _BV(PB1)) ? 1 : 0));
    PORTB &= ~_BV(PB2);
  }
  return in;
}

static uint8_t spiBytes(uint8_t cmd, const uint8_t *tx, uint8_t *rx, uint8_t len) {
  PORTB &= ~_BV(PB3);
  uint8_t status = spiByte(cmd);
  for (uint8_t i = 0; i < len; i++) {
    uint8_t value = spiByte(tx ? tx[i] : 0);
    if (rx) {
      rx[i] = value;
    }
  }
  PORTB |= _BV(PB3);
  PORTB &= ~(_BV(PB0) | _BV(PB2));
  return status;
}

static bool radioConfigure() {
  uint8_t cfg[10];
  cfg[0] = (uint8_t)(RADIO_CHANNEL & 0xFF);
  cfg[1] = (uint8_t)(((RADIO_CHANNEL >> 8) & 0x01) | (RADIO_HFREQ_PLL ? 0x02 : 0x00));
  cfg[2] = (uint8_t)((ADDR_LEN << 4) | ADDR_LEN);
  cfg[3] = PAYLOAD_LEN;
  cfg[4] = PAYLOAD_LEN;
  cfg[5] = RADIO_ADDR[0];
  cfg[6] = RADIO_ADDR[1];
  cfg[7] = RADIO_ADDR[2];
  cfg[8] = RADIO_ADDR[3];
  cfg[9] = (uint8_t)(RADIO_CRC16 | RADIO_XOF_16MHZ);

  // PWR_UP and TRX_CE are tied high, TX_EN is tied low, so the chip stays in RX.
  spiBytes(CMD_W_CONFIG, cfg, 0, 10);
  spiBytes(CMD_W_TX_ADDRESS, RADIO_ADDR, 0, ADDR_LEN);

  uint8_t back[10];
  spiBytes(CMD_R_CONFIG, 0, back, 10);
  back[1] &= 0x3F;
  for (uint8_t i = 0; i < 10; i++) {
    if (back[i] != cfg[i]) {
      return false;
    }
  }
  uint8_t addr[ADDR_LEN];
  spiBytes(CMD_R_TX_ADDRESS, 0, addr, ADDR_LEN);
  for (uint8_t i = 0; i < ADDR_LEN; i++) {
    if (addr[i] != RADIO_ADDR[i]) {
      return false;
    }
  }
  sawDrLow = true;
  return true;
}

static bool radioBegin() {
  // PB0 MOSI, PB2 SCK, PB3 CSN, PB4 RadioIn drive. PB1 MISO is an input.
  DDRB |= _BV(PB0) | _BV(PB2) | _BV(PB3) | _BV(PB4);
  DDRB &= ~_BV(PB1);
  PORTB &= ~(_BV(PB0) | _BV(PB1) | _BV(PB2) | _BV(PB4));
  PORTB |= _BV(PB3);
  // Supply rise is PWR_UP. The nRF905 crystal needs a few milliseconds.
  delay(RADIO_OSC_START_MS);
  return radioConfigure();
}

static void pollRadio() {
  if (!radioOk) {
    if ((long)(millis() - radioRetryAt) >= 0) {
      radioOk = radioConfigure();
      if (!radioOk) {
        radioRetryAt = millis() + RADIO_RETRY_MS;
      }
    }
    return;
  }

  uint8_t status = spiBytes(CMD_R_CONFIG, 0, 0, 0);
  if (!(status & STATUS_DR)) {
    sawDrLow = true;
    return;
  }
  if (!sawDrLow) {
    return;
  }
  sawDrLow = false;

  uint8_t payload[PAYLOAD_LEN];
  // Clocking the payload out clears DR while TRX_CE stays high.
  spiBytes(CMD_R_RX_PAYLOAD, 0, payload, PAYLOAD_LEN);

  char key = (char)payload[0];
  char edge = (char)payload[1];
  if (wordIndex(key) < 0 || (edge != 'D' && edge != 'U')) {
    return;
  }
  onPacket(key, edge);
}

static void chooseNext() {
  unsigned long nowMs = millis();
  if (activeKey && !stopRequested && (nowMs - activeRxMs) > HOLD_MS) {
    stopRequested = true;
  }

  if (stopRequested && activeKey == 'Y') {
    char next = queuedKey;
    queuedKey = 0;
    activeKey = 0;
    stopRequested = false;
    if (next) {
      activeKey = next;
      activeRxMs = nowMs;
    }
  }

  if (activeKey && !stopRequested) {
    int8_t index = wordIndex(activeKey);
    if (index >= 0) {
      const OokWord &word = WORDS[index];
      startFrame(word.repeat, word.nbits, false);
    }
    return;
  }

  if (stopRequested && activeKey) {
    int8_t index = wordIndex(activeKey);
    if (index >= 0 && WORDS[index].release != 0) {
      const OokWord &word = WORDS[index];
      startFrame(word.release, word.nbits, true);
      return;
    }
    activeKey = 0;
    stopRequested = false;
  }
}

static void serviceBurst(unsigned long now) {
  while ((long)(now - bitDeadline) >= 0) {
    bitIndex++;
    if (bitIndex >= burstBits) {
      writeCarrier(false);
      phase = PHASE_GAP;
      return;
    }
    writeCarrier(wordBit(burstWord, burstBits, bitIndex));
    bitDeadline += BIT_US;
  }
}

void setup() {
  radioOk = radioBegin();
  if (!radioOk) {
    radioRetryAt = millis() + RADIO_RETRY_MS;
  }
  writeCarrier(false);
}

void loop() {
  unsigned long now = micros();
  if (phase == PHASE_BURST) {
    serviceBurst(now);
    return;
  }
  if (phase == PHASE_GAP) {
    if ((long)(now - frameDeadline) >= 0) {
      finishFrame();
    } else if (radioOk && (long)(frameDeadline - now) > (long)SPI_GAP_GUARD_US) {
      pollRadio();
      return;
    } else {
      return;
    }
  }

  pollRadio();
  if (phase == PHASE_IDLE) {
    chooseNext();
  }
}
