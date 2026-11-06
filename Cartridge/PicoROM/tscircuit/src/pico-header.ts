/**
 * Raspberry Pi Pico 40-pin header, datasheet pin numbers.
 * Pin 1 is GP0 at the USB end of the left header. Pin 40 is VBUS
 * at the USB end of the right header. GPIO23 (SMPS), GPIO24 (VBUS
 * sense), GPIO25 (LED), and GPIO29 (VSYS sense) are not header pins.
 * Source: Raspberry Pi Pico datasheet pinout and pico-sdk boards/pico.h
 * (PICO_SMPS_MODE_PIN 23, PICO_VBUS_PIN 24, PICO_DEFAULT_LED_PIN 25,
 * PICO_VSYS_PIN 29).
 */
export const picoHeaderPins: ReadonlyArray<readonly [number, string]> = [
  [1, "GP0"],
  [2, "GP1"],
  [3, "GND3"],
  [4, "GP2"],
  [5, "GP3"],
  [6, "GP4"],
  [7, "GP5"],
  [8, "GND8"],
  [9, "GP6"],
  [10, "GP7"],
  [11, "GP8"],
  [12, "GP9"],
  [13, "GND13"],
  [14, "GP10"],
  [15, "GP11"],
  [16, "GP12"],
  [17, "GP13"],
  [18, "GND18"],
  [19, "GP14"],
  [20, "GP15"],
  [21, "GP16"],
  [22, "GP17"],
  [23, "GND23"],
  [24, "GP18"],
  [25, "GP19"],
  [26, "GP20"],
  [27, "GP21"],
  [28, "GND28"],
  [29, "GP22"],
  [30, "RUN"],
  [31, "GP26"],
  [32, "GP27"],
  [33, "AGND"],
  [34, "GP28"],
  [35, "ADC_VREF"],
  [36, "3V3"],
  [37, "3V3_EN"],
  [38, "GND38"],
  [39, "VSYS"],
  [40, "VBUS"],
]

export const picoPinLabels = Object.fromEntries(
  picoHeaderPins.map(([pin, name]) => [`pin${pin}`, name]),
)
