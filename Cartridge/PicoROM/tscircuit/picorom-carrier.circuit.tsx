/**
 * Maxx Steele cartridge carrier for an upstream PicoROM-28 module.
 *
 * The module is wickerwaka/PicoROM PicoROM-28 rev 1.10
 * (commit dddab41f9a47c7666115837b50b93eb78c221e33). Its DIP-28 pins are
 * the 27C512 map used by firmware target P28_27C512_70ns (BOARD_28P).
 * This sheet does not redraw the module's RP2040, regulators, or buffers.
 *
 * No external 74HCT245. On that PCB, U10 (MPN C548858, Nexperia
 * 74LVC8T245BQ) already drives the DIP data pins from +5V_SYS, and the
 * address /CE /OE 74LVC245s take 5 V inputs. A second buffer would be
 * another tri-state stage on the same pins.
 *
 * J400 names are the chosen reference map. They are not continuity-tested.
 * /CE is strapped low because that map has no chip-select finger. Stock
 * firmware enables the data driver only when both /CE and /OE are low.
 */
import { p28PinLabels } from "./src/p28-pinout"
import { j400PinLabels, j400Pins } from "./src/j400-edge"

const Join = ({
  net,
  pins,
}: {
  net: string
  pins: string[]
}) => <netlabel net={net} connectsTo={pins} inline />

const addr = [
  "A0",
  "A1",
  "A2",
  "A3",
  "A4",
  "A5",
  "A6",
  "A7",
  "A8",
  "A9",
  "A10",
  "A11",
  "A12",
  "A13",
  "A14",
  "A15",
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
    title="Maxx Steele PicoROM-28 carrier — PRELIMINARY, not fabrication-ready"
  >
    <schematicsheet
      name="carrier"
      displayName="PicoROM-28 wired to J400"
      sheetIndex={0}
      sheetWidth="200mm"
      sheetHeight="110mm"
    />

    <schematicsection name="mod" displayName="Upstream PicoROM-28 module" />
    <schematicsection name="edge" displayName="J400 reference map" />

    <chip
      name="U_P28"
      manufacturerPartNumber="PicoROM-P28-rev1.10"
      footprint="dip28"
      schSheetName="carrier"
      schSectionName="mod"
      schX={-6}
      schY={0}
      schWidth={3.2}
      schFacingDirection="right"
      pcbX={-22}
      pcbY={0}
      pinLabels={p28PinLabels}
      pinAttributes={{
        VCC: { requiresPower: true },
        GND: { requiresGround: true },
      }}
      schPinArrangement={{
        leftSide: {
          direction: "top-to-bottom",
          pins: ["nCE"],
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
      schX={6}
      schY={0}
      pcbX={22}
      pcbY={0}
      pcbRotation={90}
      schWidth={2.4}
      pinLabels={j400PinLabels}
      schFacingDirection="left"
    />

    {addr.map((name) => (
      <Join key={name} net={`J400_${name}`} pins={[`U_P28.${name}`, `J1.${name}`]} />
    ))}
    {data.map((name) => (
      <Join key={name} net={`J400_${name}`} pins={[`U_P28.${name}`, `J1.${name}`]} />
    ))}
    <Join net="J400_OEB" pins={["U_P28.nOE", "J1.OEB"]} />
    <Join
      net="GND"
      pins={["U_P28.nCE", "U_P28.GND", "J1.GND_1", "J1.GND_43"]}
    />
    <Join net="VSW_ASSUMED_5V" pins={["U_P28.VCC", "J1.VSW"]} />

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
      schY={5.4}
      anchor="center"
      color="#9b1c1c"
      fontSize={0.28}
      text="WIRES JOIN U_P28 TO J400. NOT A CONTINUITY TEST."
    />
    <schematictext
      schSheetName="carrier"
      schSectionName="edge"
      schX={0}
      schY={-5.4}
      anchor="center"
      fontSize={0.18}
      text="A0-A15 and D0-D7 are wired. OEB is /OE. /CE is GND. VSW is pin 28. NC_ labels do not touch the module."
    />
  </board>
)
