/**
 * J400 expansion-port line numbers from the community reference guide
 * pinout table (revision AHj4eMTerV6p8oM8x4ORN-S7N5q-3zin1iMaLm2nH32DLN3-UDgSbYavvxt2Mc04yVAXFFyXkbVy76Jb0n3s9g).
 * The guide says the table "needs to be verified". These are schematic
 * line numbers, not the silk order on the cartridge photo.
 *
 * Source names with spaces or mixed case are stored as pin labels that
 * tscircuit can trace. The guide's original spelling is in J400-PINOUT.md.
 */
export const j400Pins: ReadonlyArray<readonly [number, string]> = [
  [1, "GND_1"],
  [2, "A15"],
  [3, "A14"],
  [4, "A13"],
  [5, "A12"],
  [6, "A11"],
  [7, "A7"],
  [8, "A8"],
  [9, "A6"],
  [10, "A9"],
  [11, "A5"],
  [12, "OEB"],
  [13, "A4"],
  [14, "WRB"],
  [15, "A3"],
  [16, "A10"],
  [17, "A2"],
  [18, "A1"],
  [19, "D7"],
  [20, "A0"],
  [21, "D6"],
  [22, "D0"],
  [23, "D5"],
  [24, "D1"],
  [25, "D4"],
  [26, "D2"],
  [27, "AUDIO_IN"],
  [28, "D3"],
  [29, "F2"],
  [30, "SA0"],
  [31, "WEB"],
  [32, "SA1"],
  [33, "BIGSPACEB"],
  [34, "SA2"],
  [35, "MNMIB"],
  [36, "SA3"],
  [37, "RESETSYSB"],
  [38, "CT"],
  [39, "VBAT"],
  [40, "DOUT"],
  [41, "VSW"],
  [42, "DREQ"],
  [43, "GND_43"],
  [44, "ASTRB"],
]

export const j400PinLabels = Object.fromEntries(
  j400Pins.map(([pin, name]) => [`pin${pin}`, name]),
)
