/**
 * DIP-28 pin functions taken from PicoROM-28 PCB pad nets
 * (hardware/PicoROM-28/PicoROM-28.kicad_pcb at commit dddab41).
 * J3 pin N is DIP pin N. J4 pin 1 is DIP pin 28, counting downward.
 * Pin 1 is A15 and pin 28 is VCC, the 27C512 map, not the 27C256 VPP/WE map.
 */
export const p28PinLabels = {
  pin1: "A15",
  pin2: "A12",
  pin3: "A7",
  pin4: "A6",
  pin5: "A5",
  pin6: "A4",
  pin7: "A3",
  pin8: "A2",
  pin9: "A1",
  pin10: "A0",
  pin11: "D0",
  pin12: "D1",
  pin13: "D2",
  pin14: "GND",
  pin15: "D3",
  pin16: "D4",
  pin17: "D5",
  pin18: "D6",
  pin19: "D7",
  pin20: "nCE",
  pin21: "A10",
  pin22: "nOE",
  pin23: "A11",
  pin24: "A9",
  pin25: "A8",
  pin26: "A13",
  pin27: "A14",
  pin28: "VCC",
} as const
