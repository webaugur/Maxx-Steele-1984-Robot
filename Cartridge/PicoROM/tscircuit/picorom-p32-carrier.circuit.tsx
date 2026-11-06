/**
 * Maxx Steele cartridge carrier for an upstream PicoROM-32 module.
 *
 * Module: wickerwaka/PicoROM PicoROM-32 rev 1.10
 * (commit dddab41f9a47c7666115837b50b93eb78c221e33).
 * Firmware target P32_27C020_70ns (BOARD_32P). ROM_SIZE is 256 KB,
 * address GPIO0–17 and data GPIO18–25. A16 and A17 are tied low here,
 * so the robot's A0–A15 select the first 64 KB of that image.
 * There is no bank-switch circuit.
 *
 * J400 is the chosen reference map. It is not continuity-tested.
 * /CE is strapped low. The module ORs /CE with /OE before the RP2040
 * samples that one enable. OEB is /OE.
 *
 * No external HCT. On this PCB, U10 (MPN C548858, Nexperia 74LVC8T245BQ)
 * drives the DIP data pins from +5V_SYS, and its DIR pin is tied to 3.3 V.
 */
import { p32PinLabels } from "./src/p32-pinout"
import { j400PinLabels, j400Pins } from "./src/j400-edge"

const Join = ({ net, pins }: { net: string; pins: string[] }) => (
  <netlabel net={net} connectsTo={pins} inline />
)

const addr = [
  "A0", "A1", "A2", "A3", "A4", "A5", "A6", "A7",
  "A8", "A9", "A10", "A11", "A12", "A13", "A14", "A15",
] as const
const data = ["D0", "D1", "D2", "D3", "D4", "D5", "D6", "D7"] as const

const connectedJ1 = new Set<string>([
  ...addr,
  ...data,
  "OEB",
  "VSW",
  "GND_1",
  "GND_43",
])

export default () => (
  <board
    width="90mm"
    height="130mm"
    routingDisabled
    title="Maxx Steele PicoROM-32 carrier — PRELIMINARY, not fabrication-ready"
  >
    <schematicsheet
      name="carrier"
      displayName="PicoROM-32 wired to J400"
      sheetIndex={0}
      sheetWidth="210mm"
      sheetHeight="120mm"
    />
    <schematicsection name="mod" displayName="Upstream PicoROM-32 module" />
    <schematicsection name="edge" displayName="J400 reference map" />

    <chip
      name="U_P32"
      manufacturerPartNumber="PicoROM-P32-rev1.10"
      footprint="dip32"
      schSheetName="carrier"
      schSectionName="mod"
      schX={-6}
      schY={0}
      schWidth={3.4}
      schFacingDirection="right"
      pcbX={-22}
      pcbY={0}
      pinLabels={p32PinLabels}
      pinAttributes={{
        VCC: { requiresPower: true },
        GND: { requiresGround: true },
      }}
      schPinArrangement={{
        leftSide: {
          direction: "top-to-bottom",
          pins: ["VPP", "PGM", "A16", "A17", "nCE"],
        },
        rightSide: {
          direction: "top-to-bottom",
          pins: [...addr, ...data, "nOE"],
        },
        topSide: { direction: "left-to-right", pins: ["VCC"] },
        bottomSide: { direction: "left-to-right", pins: ["GND"] },
      }}
    />

    <pinheader
      name="J1"
      pinCount={44}
      pitch="2.54mm"
      gender="female"
      schSheetName="carrier"
      schSectionName="edge"
      schX={6.2}
      schY={0}
      pcbX={22}
      pcbY={0}
      pcbRotation={90}
      schWidth={2.6}
      pinLabels={j400PinLabels}
      schFacingDirection="left"
    />

    {addr.map((name) => (
      <Join key={name} net={`J400_${name}`} pins={[`U_P32.${name}`, `J1.${name}`]} />
    ))}
    {data.map((name) => (
      <Join key={name} net={`J400_${name}`} pins={[`U_P32.${name}`, `J1.${name}`]} />
    ))}
    <Join net="J400_OEB" pins={["U_P32.nOE", "J1.OEB"]} />
    <Join
      net="GND"
      pins={["U_P32.nCE", "U_P32.A16", "U_P32.A17", "U_P32.GND", "J1.GND_1", "J1.GND_43"]}
    />
    <Join net="VSW_ASSUMED_5V" pins={["U_P32.VCC", "J1.VSW"]} />
    <Join net="NC_VPP" pins={["U_P32.VPP"]} />
    <Join net="NC_PGM" pins={["U_P32.PGM"]} />

    {j400Pins
      .filter(([, name]) => !connectedJ1.has(name))
      .map(([, name]) => (
        <Join
          key={name}
          net={name === "VBAT" ? "J400_VBAT_NOT_LOGIC" : `NC_${name}`}
          pins={[`J1.${name}`]}
        />
      ))}

    <schematictext
      schSheetName="carrier"
      schSectionName="mod"
      schX={0}
      schY={5.6}
      anchor="center"
      color="#9b1c1c"
      fontSize={0.26}
      text="P32 WIRES TO J400. A16 AND A17 TIED LOW. NOT A CONTINUITY TEST."
    />
    <schematictext
      schSheetName="carrier"
      schSectionName="edge"
      schX={0}
      schY={-5.6}
      anchor="center"
      fontSize={0.16}
      text="A0-A15 and D0-D7 are wired. OEB is /OE. /CE, A16, A17 are GND. VSW is pin 32. VPP and PGM are NC."
    />
  </board>
)
