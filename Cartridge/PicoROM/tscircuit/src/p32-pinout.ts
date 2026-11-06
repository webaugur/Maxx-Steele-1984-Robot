/**
 * DIP-32 pin functions from PicoROM-32 rev 1.10 PCB headers.
 * wickerwaka/PicoROM commit dddab41f9a47c7666115837b50b93eb78c221e33.
 * J3 pin 1 is DIP pin 1. J4 pin 1 is DIP pin 32, then the DIP number counts down.
 * This is the 27C020 map used by firmware target P32_27C020_70ns, not a 27C256.
 * Pin 1 and pin 31 have header pads and no copper net on that PCB.
 */
export const p32PinLabels = {
  pin1: "VPP",
  pin2: "A16",
  pin3: "A15",
  pin4: "A12",
  pin5: "A7",
  pin6: "A6",
  pin7: "A5",
  pin8: "A4",
  pin9: "A3",
  pin10: "A2",
  pin11: "A1",
  pin12: "A0",
  pin13: "D0",
  pin14: "D1",
  pin15: "D2",
  pin16: "GND",
  pin17: "D3",
  pin18: "D4",
  pin19: "D5",
  pin20: "D6",
  pin21: "D7",
  pin22: "nCE",
  pin23: "A10",
  pin24: "nOE",
  pin25: "A11",
  pin26: "A9",
  pin27: "A8",
  pin28: "A13",
  pin29: "A14",
  pin30: "A17",
  pin31: "PGM",
  pin32: "VCC",
} as const

/** DIP pin number for each signal name. Checked against the generated netlist. */
export const p32PinNumber: Record<string, number> = Object.fromEntries(
  Object.entries(p32PinLabels).map(([pin, name]) => [name, Number(pin.slice(3))]),
)
