// Maxx Steele remote: factory keypad -> nRF905.
// Wiring, channel, and payload: Transmitter/Firmware/Arduino/README.md
//
// Bare ATmega328P at 3.3 V, 8 MHz crystal. The nRF905 is not 5 V tolerant.
// PWR_UP is tied to 3.3 V. J2 pin 1 is +9 V and feeds the regulator only.

#include <SPI.h>

// Keypad. Rows are inputs with pull-ups. One column is driven low at a time;
// the others stay hi-Z so two keys cannot short two outputs together.
// Order matches keyboard-matrix-reference-1.png: columns L7 L6 L5 L4,
// rows D0 D1 L0 L1 L2 L3. Index 0 is key 'A'.
static const uint8_t ROW_PINS[] = {3, 4, 5, 6, A4, A5};  // D0 D1 L0 L1 L2 L3
static const uint8_t COL_PINS[] = {A3, A2, A1, A0};      // L7 L6 L5 L4
static const uint8_t PIN_PK = 2;                         // J2 pin 4, active low
static const uint8_t ROW_COUNT = sizeof(ROW_PINS);
static const uint8_t COL_COUNT = sizeof(COL_PINS);
static_assert(ROW_COUNT == 6, "row pin table");
static_assert(COL_COUNT == 4, "column pin table");
static const int8_t KEY_NONE = -1;
static const int8_t KEY_POWER = 24;  // 'Y'

// nRF905. DR/CD/AM/uPCLK are left open; DR is bit 5 of the SPI status byte.
static const uint8_t PIN_CSN = 10;     // PB2, DIP pin 16
static const uint8_t PIN_TRX_CE = 8;   // PB0, DIP pin 14
static const uint8_t PIN_TX_EN = 9;    // PB1, DIP pin 15

// (422.4 + channel/10) * (1 + HFREQ_PLL) MHz.
// Channel 108, HFREQ_PLL 0 -> 433.2 MHz. Set HFREQ_PLL to 1 for the 868/915 band.
static const uint16_t RADIO_CHANNEL = 108;
static const uint8_t RADIO_HFREQ_PLL = 0;
static const uint8_t RADIO_ADDR[] = {'M', 'A', 'X', 'X'};
static const uint8_t PAYLOAD_LEN = 2;
static const uint8_t ADDR_LEN = sizeof(RADIO_ADDR);
static_assert(ADDR_LEN == 4, "nRF905 address is 4 bytes");
static_assert(F_CPU == 8000000UL, "build for an 8 MHz ATmega328P");

// Config byte 9: 16-bit CRC, CRC on, 16 MHz crystal, uPCLK off.
// Change the crystal field if the module is not 16 MHz (see README).
static const uint8_t RADIO_XOF_16MHZ = 0x18;
static const uint8_t RADIO_CRC16 = 0xC0;

static const unsigned long DEBOUNCE_MS = 15;
static const unsigned long REPEAT_MS = 30;
static const unsigned long RADIO_DR_TIMEOUT_MS = 20;
static const unsigned long RADIO_RETRY_MS = 1000;
// nRF905 product spec: crystal start-up is typically 3 ms after power rises.
// PWR_UP is tied to 3.3 V, so that rise is the regulator, not a GPIO.
static const unsigned long RADIO_OSC_START_MS = 5;

static const uint8_t CMD_W_CONFIG = 0x00;
static const uint8_t CMD_R_CONFIG = 0x10;
static const uint8_t CMD_W_TX_PAYLOAD = 0x20;
static const uint8_t CMD_W_TX_ADDRESS = 0x22;
static const uint8_t CMD_R_TX_ADDRESS = 0x23;
static const uint8_t STATUS_DR = 0x20;

static int8_t stableKey = KEY_NONE;
static int8_t candidate = KEY_NONE;
static unsigned long candidateSince = 0;
static unsigned long lastRepeat = 0;
static bool radioOk = false;
static unsigned long radioRetryAt = 0;

static void columnRelease() {
  for (uint8_t i = 0; i < COL_COUNT; i++) {
    pinMode(COL_PINS[i], INPUT);
  }
}

static void columnDrive(uint8_t index) {
  columnRelease();
  pinMode(COL_PINS[index], OUTPUT);
  digitalWrite(COL_PINS[index], LOW);
}

// One pass. Power/Stop wins over the matrix. First matrix hit wins; the
// membrane has no diodes, so a second key in the same pass is ignored.
static int8_t scanRaw() {
  if (digitalRead(PIN_PK) == LOW) {
    columnRelease();
    return KEY_POWER;
  }
  for (uint8_t col = 0; col < COL_COUNT; col++) {
    columnDrive(col);
    // Internal pull-up is tens of kΩ. A few microseconds settles the row.
    delayMicroseconds(50);
    for (uint8_t row = 0; row < ROW_COUNT; row++) {
      if (digitalRead(ROW_PINS[row]) == LOW) {
        columnRelease();
        return (int8_t)(row * COL_COUNT + col);
      }
    }
  }
  columnRelease();
  return KEY_NONE;
}

static char keyLetter(int8_t key) {
  return (char)('A' + key);
}

static uint8_t spiBytes(uint8_t cmd, const uint8_t *tx, uint8_t *rx, uint8_t len) {
  SPI.beginTransaction(SPISettings(1000000, MSBFIRST, SPI_MODE0));
  digitalWrite(PIN_CSN, LOW);
  uint8_t status = SPI.transfer(cmd);
  for (uint8_t i = 0; i < len; i++) {
    uint8_t value = SPI.transfer(tx ? tx[i] : 0);
    if (rx) {
      rx[i] = value;
    }
  }
  digitalWrite(PIN_CSN, HIGH);
  SPI.endTransaction();
  return status;
}

static void radioStandby() {
  digitalWrite(PIN_TRX_CE, LOW);
  digitalWrite(PIN_TX_EN, LOW);
}

static bool radioConfigure() {
  uint8_t cfg[10];
  cfg[0] = (uint8_t)(RADIO_CHANNEL & 0xFF);
  cfg[1] = (uint8_t)(((RADIO_CHANNEL >> 8) & 0x01) | (RADIO_HFREQ_PLL ? 0x02 : 0x00));
  // PA −10 dBm, no auto-retransmit: bits 5:2 stay 0.
  cfg[2] = (uint8_t)((ADDR_LEN << 4) | ADDR_LEN);
  cfg[3] = PAYLOAD_LEN;
  cfg[4] = PAYLOAD_LEN;
  cfg[5] = RADIO_ADDR[0];
  cfg[6] = RADIO_ADDR[1];
  cfg[7] = RADIO_ADDR[2];
  cfg[8] = RADIO_ADDR[3];
  cfg[9] = (uint8_t)(RADIO_CRC16 | RADIO_XOF_16MHZ);

  radioStandby();
  spiBytes(CMD_W_CONFIG, cfg, 0, 10);
  spiBytes(CMD_W_TX_ADDRESS, RADIO_ADDR, 0, ADDR_LEN);

  uint8_t back[10];
  spiBytes(CMD_R_CONFIG, 0, back, 10);
  // Config byte 1 bits 7:6 are unused and are not compared.
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
  return true;
}

static bool radioBegin() {
  pinMode(PIN_CSN, OUTPUT);
  pinMode(PIN_TRX_CE, OUTPUT);
  pinMode(PIN_TX_EN, OUTPUT);
  digitalWrite(PIN_CSN, HIGH);
  radioStandby();
  delay(RADIO_OSC_START_MS);

  SPI.begin();
  bool ok = radioConfigure();
  if (!ok) {
    Serial.println(F("nRF905 config readback mismatch"));
  }
  return ok;
}

// ShockBurst. Returns false if the radio failed init or DR does not rise.
static bool radioSend(int8_t key, char edge) {
  if (!radioOk) {
    return false;
  }
  uint8_t payload[PAYLOAD_LEN];
  payload[0] = (uint8_t)keyLetter(key);
  payload[1] = (uint8_t)edge;

  radioStandby();
  spiBytes(CMD_W_TX_PAYLOAD, payload, 0, PAYLOAD_LEN);
  digitalWrite(PIN_TX_EN, HIGH);
  digitalWrite(PIN_TRX_CE, HIGH);

  // DR goes high when ShockBurst finishes. Wait until it has been low in
  // this burst, then high, so a stuck status bit cannot count as success.
  unsigned long start = millis();
  bool sawLow = false;
  while ((millis() - start) < RADIO_DR_TIMEOUT_MS) {
    // Command byte only. R_CONFIG with no following bytes returns status on MISO.
    uint8_t status = spiBytes(CMD_R_CONFIG, 0, 0, 0);
    if (status & STATUS_DR) {
      if (sawLow) {
        radioStandby();
        return true;
      }
    } else {
      sawLow = true;
    }
  }
  radioStandby();
  Serial.println(F("nRF905 DR timeout"));
  return false;
}

static void noteEdge(int8_t key, char edge) {
  if (key == KEY_NONE) {
    return;
  }
  Serial.write(keyLetter(key));
  Serial.write(edge);
  Serial.println();
  radioSend(key, edge);
}

void setup() {
  Serial.begin(9600);
  pinMode(PIN_PK, INPUT_PULLUP);
  for (uint8_t i = 0; i < ROW_COUNT; i++) {
    pinMode(ROW_PINS[i], INPUT_PULLUP);
  }
  columnRelease();
  radioOk = radioBegin();
  if (!radioOk) {
    radioRetryAt = millis() + RADIO_RETRY_MS;
  }
}

void loop() {
  if (!radioOk && (long)(millis() - radioRetryAt) >= 0) {
    radioOk = radioConfigure();
    if (!radioOk) {
      Serial.println(F("nRF905 config readback mismatch"));
      radioRetryAt = millis() + RADIO_RETRY_MS;
    }
  }

  int8_t raw = scanRaw();
  unsigned long now = millis();
  if (raw != candidate) {
    candidate = raw;
    candidateSince = now;
  } else if (raw != stableKey && (now - candidateSince) >= DEBOUNCE_MS) {
    if (stableKey != KEY_NONE) {
      noteEdge(stableKey, 'U');
    }
    stableKey = raw;
    if (stableKey != KEY_NONE) {
      noteEdge(stableKey, 'D');
      lastRepeat = now;
    }
  } else if (stableKey != KEY_NONE && (now - lastRepeat) >= REPEAT_MS) {
    noteEdge(stableKey, 'D');
    lastRepeat = now;
  }
}
