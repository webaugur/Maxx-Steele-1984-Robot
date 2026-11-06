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

const Trace = ({ from, to }: { from: string; to: string }) => (
  <trace from={from} to={to} />
)

const share = (p28Pin: string, j1Pin: string, net: string) => [
  <Trace key={`p28-${net}`} from={`U_P28.${p28Pin}`} to={`net.${net}`} />,
  <Trace key={`j1-${net}`} from={`J1.${j1Pin}`} to={`net.${net}`} />,
]

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
      displayName="1 PicoROM-28 on the J400 map"
      sheetIndex={0}
      sheetWidth="220mm"
      sheetHeight="180mm"
    />
    <schematicsheet
      name="notes"
      displayName="2 Module power and what this sheet is not"
      sheetIndex={1}
      sheetWidth="190mm"
      sheetHeight="90mm"
    />

    <schematicsection name="mod" displayName="Upstream PicoROM-28 module" />
    <schematicsection name="edge" displayName="J400 reference map" />
    <schematicsection name="notesec" displayName="Checks are not a power-up" />

    <chip
      name="U_P28"
      manufacturerPartNumber="PicoROM-P28-rev1.10"
      footprint="dip28"
      schSheetName="carrier"
      schSectionName="mod"
      schX={-1.2}
      schY={0}
      schWidth={1.77}
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
          pins: [...addr],
        },
        rightSide: {
          direction: "top-to-bottom",
          pins: [...data, "nCE", "nOE"],
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
      schX={3.2}
      schY={0}
      pcbX={22}
      pcbY={0}
      pcbRotation={90}
      schWidth={1.15}
      pinLabels={j400PinLabels}
      schFacingDirection="right"
    />

    {addr.flatMap((name) => share(name, name, `J400_${name}`))}
    {data.flatMap((name) => share(name, name, `J400_${name}`))}
    {share("nOE", "OEB", "J400_OEB")}
    <Trace from="U_P28.nCE" to="net.GND" />
    <Trace from="U_P28.VCC" to="net.VSW_ASSUMED_5V" />
    <Trace from="J1.VSW" to="net.VSW_ASSUMED_5V" />
    <Trace from="U_P28.GND" to="net.GND" />
    <Trace from="J1.GND_1" to="net.GND" />
    <Trace from="J1.GND_43" to="net.GND" />

    {j400Pins
      .filter(([, name]) => !connectedJ1.has(name))
      .map(([, name]) => (
        <Trace
          key={name}
          from={`J1.${name}`}
          to={`net.${name === "VBAT" ? "J400_VBAT_NOT_LOGIC" : `J400_${name}_UNUSED`}`}
        />
      ))}

    <schematictext
      schSheetName="carrier"
      schSectionName="mod"
      schX={-2}
      schY={5.2}
      anchor="center"
      color="#9b1c1c"
      fontSize={0.28}
      text="PRELIMINARY. NOT A PCB. NOT CONTINUITY-TESTED."
    />
    <schematictext
      schSheetName="carrier"
      schSectionName="mod"
      schX={-2}
      schY={-5.2}
      anchor="center"
      fontSize={0.18}
      text="U_P28 is PicoROM-28 rev 1.10. /CE strapped low. OEB is /OE. No external HCT."
    />

    <schematictext
      schSheetName="notes"
      schSectionName="notesec"
      schX={0}
      schY={2.2}
      anchor="center"
      color="#9b1c1c"
      fontSize={0.28}
      text="NO EXTERNAL HCT245. BUFFERS ARE INSIDE THE MODULE."
    />
    <schematictext
      schSheetName="notes"
      schSectionName="notesec"
      schX={0}
      schY={1.2}
      anchor="center"
      fontSize={0.18}
      text="ASSUMED VSW = 5 V on pin 28. Confirm 4.5-5.5 V, spikes included, before power. No 7805. VBAT open."
    />
    <schematictext
      schSheetName="notes"
      schSectionName="notesec"
      schX={0}
      schY={0.4}
      anchor="center"
      fontSize={0.18}
      text="Module Schottkys OR USB and pin 28. This sheet does not short USB to VSW."
    />
    <schematictext
      schSheetName="notes"
      schSectionName="notesec"
      schX={0}
      schY={-0.6}
      anchor="center"
      fontSize={0.18}
      text="Stock firmware P28_27C512_70ns. Address GPIO0-15, data GPIO16-23. Not a bare Pico map."
    />
    <schematictext
      schSheetName="notes"
      schSectionName="notesec"
      schX={0}
      schY={-1.6}
      anchor="center"
      fontSize={0.18}
      text="Upstream quotes 70 ns / 40 ns on its own bench. This repo has no robot scope capture."
    />
  </board>
)
