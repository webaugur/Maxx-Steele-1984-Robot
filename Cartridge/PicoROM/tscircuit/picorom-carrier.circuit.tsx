/**
 * Preliminary Maxx Steele cartridge carrier for an upstream PicoROM P28 module.
 *
 * This file is a schematic of known interfaces. It is not a PCB layout and it
 * is not fabrication-ready. J1 is not wired to the P28. No 74-series driver is
 * instantiated: upstream P28 already buffers the ROM pins, and the on-hand
 * 7400 family/part is still unidentified.
 *
 * Pin functions on U_P28 are the DIP-28 nets on PicoROM-28 rev 1.10
 * (wickerwaka/PicoROM commit dddab41f9a47c7666115837b50b93eb78c221e33):
 * J3 pins 1–14 and J4 pins 14–1. See CONNECTION-INVENTORY.md.
 */
import { p28PinLabels, p28OpenNets } from "./src/p28-pinout"
import { j1PinLabels, j1OpenNets } from "./src/j1-unresolved"

const OpenTrace = ({ from, to }: { from: string; to: string }) => (
  <trace from={from} to={to} />
)

const openTraces = (
  ref: string,
  nets: ReadonlyArray<readonly [string, string]>,
) =>
  nets.map(([pin, net]) => (
    <OpenTrace
      key={`${ref}.${pin}`}
      from={`${ref}.${pin}`}
      to={`net.${net}`}
    />
  ))

export default () => (
  <board
    width="90mm"
    height="130mm"
    routingDisabled
    title="Maxx Steele PicoROM carrier — PRELIMINARY, not fabrication-ready"
  >
    <schematicsheet
      name="p28"
      displayName="1 P28 module pins (open)"
      sheetIndex={0}
      sheetWidth="130mm"
      sheetHeight="120mm"
    />
    <schematicsheet
      name="j1"
      displayName="2 J1 edge (all unresolved)"
      sheetIndex={1}
      sheetWidth="160mm"
      sheetHeight="150mm"
    />
    <schematicsheet
      name="status"
      displayName="3 Status, buffers, blockers"
      sheetIndex={2}
      sheetWidth="190mm"
      sheetHeight="130mm"
    />

    <schematicsection name="p28mod" displayName="Upstream PicoROM P28 module" />
    <schematicsection name="edge" displayName="Maxx edge J1 — unresolved" />
    <schematicsection name="statusnotes" displayName="Do not fabricate from this sheet" />

    <chip
      name="U_P28"
      manufacturerPartNumber="PicoROM-P28-rev1.10"
      footprint="dip28"
      schSheetName="p28"
      schSectionName="p28mod"
      schX={0}
      schY={0}
      schWidth={8}
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
          pins: [
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
          ],
        },
        rightSide: {
          direction: "top-to-bottom",
          pins: ["D0", "D1", "D2", "D3", "D4", "D5", "D6", "D7", "nCE", "nOE"],
        },
        topSide: { direction: "left-to-right", pins: ["VCC"] },
        bottomSide: { direction: "left-to-right", pins: ["GND"] },
      }}
    />
    {openTraces("U_P28", p28OpenNets)}

    <schematictext
      schSheetName="p28"
      schSectionName="p28mod"
      schX={0}
      schY={3.6}
      anchor="center"
      color="#9b1c1c"
      fontSize={0.35}
      text="PRELIMINARY — NOT FABRICATION-READY"
    />
    <schematictext
      schSheetName="p28"
      schSectionName="p28mod"
      schX={0}
      schY={3.0}
      anchor="center"
      color="#9b1c1c"
      fontSize={0.22}
      text="Each P28 pin is its own open net. Nothing here connects to J1."
    />
    <schematictext
      schSheetName="p28"
      schSectionName="p28mod"
      schX={0}
      schY={-3.5}
      anchor="center"
      fontSize={0.22}
      text="U_P28 is the upstream DIP-28 module (integrated RP2040). Not a Pi Pico. Not a new EPROM replica."
    />

    <pinheader
      name="J1"
      pinCount={44}
      pitch="2.54mm"
      gender="female"
      schSheetName="j1"
      schSectionName="edge"
      schX={0}
      schY={0}
      pcbX={18}
      pcbY={0}
      pcbRotation={90}
      schWidth={0.675}
      pinLabels={j1PinLabels}
      schFacingDirection="right"
    />
    {openTraces("J1", j1OpenNets)}

    <schematictext
      schSheetName="j1"
      schSectionName="edge"
      schX={0}
      schY={6.4}
      anchor="center"
      color="#9b1c1c"
      fontSize={0.28}
      text="J1 UNRESOLVED. DO NOT ROUTE."
    />
    <schematictext
      schSheetName="j1"
      schSectionName="edge"
      schX={0}
      schY={5.7}
      anchor="center"
      fontSize={0.2}
      text="IDX pins are drawing indexes, not silk numbers and not a bus map."
    />
    <schematictext
      schSheetName="j1"
      schSectionName="edge"
      schX={0}
      schY={-6.2}
      anchor="center"
      fontSize={0.2}
      text="44 contacts, 2.54 mm, 22 per face (STEP). Signal assignment is open."
    />

    <schematictext
      schSheetName="status"
      schSectionName="statusnotes"
      schX={0}
      schY={3.4}
      anchor="center"
      color="#9b1c1c"
      fontSize={0.4}
      text="NO EXTRA 7400 DRIVER IN THIS SCHEMATIC"
    />
    <schematictext
      schSheetName="status"
      schSectionName="statusnotes"
      schX={0}
      schY={2.6}
      anchor="center"
      fontSize={0.24}
      text="Upstream P28 rev 1.10 already buffers ROM pins inside the module."
    />
    <schematictext
      schSheetName="status"
      schSectionName="statusnotes"
      schX={0}
      schY={1.9}
      anchor="center"
      fontSize={0.22}
      text="Address/control: U4 U5 U9 value 74LVC245, VCC +3V3_SYS, datasheet link SN74LVC245A."
    />
    <schematictext
      schSheetName="status"
      schSectionName="statusnotes"
      schX={0}
      schY={1.2}
      anchor="center"
      fontSize={0.22}
      text="Data: U10 MPN field C548858 = Nexperia 74LVC8T245BQ. A=+3V3_SYS, B=+5V_SYS."
    />
    <schematictext
      schSheetName="status"
      schSectionName="statusnotes"
      schX={0}
      schY={0.5}
      anchor="center"
      fontSize={0.22}
      text="Reset: U7 U11 74AHCT1G126 on +5V_SYS. Not a substitute for an unidentified 74xx part."
    />
    <schematictext
      schSheetName="status"
      schSectionName="statusnotes"
      schX={0}
      schY={-0.2}
      anchor="center"
      fontSize={0.22}
      text="On-hand 7400 line drivers are a candidate only. Family and part number are pending."
    />
    <schematictext
      schSheetName="status"
      schSectionName="statusnotes"
      schX={0}
      schY={-0.9}
      anchor="center"
      fontSize={0.22}
      text="RP2040 GPIO absolute max is IOVDD+0.5 V. Do not wire the robot bus straight to a Pico."
    />
    <schematictext
      schSheetName="status"
      schSectionName="statusnotes"
      schX={0}
      schY={-1.6}
      anchor="center"
      fontSize={0.22}
      text="Stock P28 firmware uses GPIO23, GPIO24, GPIO25, and GPIO29. A Pi Pico does not break those out."
    />
    <schematictext
      schSheetName="status"
      schSectionName="statusnotes"
      schX={0}
      schY={-2.3}
      anchor="center"
      fontSize={0.22}
      text="PCB coordinates in this file are placeholders. routingDisabled. No fab outputs."
    />
    <schematictext
      schSheetName="status"
      schSectionName="statusnotes"
      schX={0}
      schY={-3.0}
      anchor="center"
      fontSize={0.22}
      text="Tooling checks are not electrical proof. Issues #46 #50 #49 stay open."
    />

    <pcbnotetext
      pcbX={0}
      pcbY={55}
      text="PLACEHOLDER FOOTPRINTS ONLY. NOT A CARTRIDGE PCB."
    />
  </board>
)
