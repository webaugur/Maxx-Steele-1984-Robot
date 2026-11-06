/**
 * Standard 20-pin DIP pinout for SN74LVC245A and SN74HCT245.
 * TI datasheets, PDIP (N), top view:
 *   1 DIR, 2–9 A1–A8, 10 GND, 11 B8, 12–18 B7–B1, 19 OE, 20 VCC.
 * DIR high: A→B. DIR low: B→A. OE high: both buses high-Z.
 */
export const dip245PinLabels = {
  pin1: "DIR",
  pin2: "A1",
  pin3: "A2",
  pin4: "A3",
  pin5: "A4",
  pin6: "A5",
  pin7: "A6",
  pin8: "A7",
  pin9: "A8",
  pin10: "GND",
  pin11: "B8",
  pin12: "B7",
  pin13: "B6",
  pin14: "B5",
  pin15: "B4",
  pin16: "B3",
  pin17: "B2",
  pin18: "B1",
  pin19: "nOE",
  pin20: "VCC",
} as const

export const dip245Arrangement = {
  leftSide: {
    direction: "top-to-bottom" as const,
    pins: ["DIR", "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "GND"],
  },
  rightSide: {
    direction: "top-to-bottom" as const,
    pins: ["VCC", "nOE", "B1", "B2", "B3", "B4", "B5", "B6", "B7", "B8"],
  },
}
