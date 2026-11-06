/**
 * Preliminary Maxx cartridge: Raspberry Pi Pico plus DIP-20 245 buffers,
 * wired to J400 line numbers from the community reference-guide pinout.
 *
 * That pinout is explicitly unverified. OEB is not tied to the HCT output
 * enable. VBAT is not connected. VSW is the assumed nominal 5 V rail for
 * the HCT and, through a Schottky, for Pico VSYS. Verify that rail before
 * powering. No 7805.
 * Stock PicoROM P28 firmware does not use this GPIO map. The map is a
 * proposal and has not been compiled or flashed.
 */
import { dip245Arrangement, dip245PinLabels } from "./src/dip245-pins"
import { j400PinLabels } from "./src/j400-edge"
import { picoPinLabels } from "./src/pico-header"

const OpenTrace = ({ from, to }: { from: string; to: string }) => (
  <trace from={from} to={to} />
)

const ties = (pairs: ReadonlyArray<readonly [string, string]>) =>
  pairs.map(([from, to]) => (
    <OpenTrace key={`${from}->${to}`} from={from} to={to} />
  ))

const lo = [0, 1, 2, 3, 4, 5, 6, 7] as const
const hiBits = [8, 9, 10, 11, 12, 13, 14, 15] as const

const idle = [
  "AUDIO_IN",
  "F2",
  "SA0",
  "WEB",
  "SA1",
  "BIGSPACEB",
  "SA2",
  "MNMIB",
  "SA3",
  "RESETSYSB",
  "CT",
  "DOUT",
  "DREQ",
  "ASTRB",
] as const

export default () => (
  <board
    width="170mm"
    height="150mm"
    routingDisabled
    title="Pico RP2040 + DIP 245 on J400 — PRELIMINARY"
  >
    <schematicsheet name="pico" displayName="1 Pico header" sheetIndex={0} sheetWidth="90mm" sheetHeight="220mm" />
    <schematicsheet name="lvc" displayName="2 LVC245 into the Pico" sheetIndex={1} sheetWidth="280mm" sheetHeight="120mm" />
    <schematicsheet name="hct" displayName="3 HCT245 data toward J400" sheetIndex={2} sheetWidth="220mm" sheetHeight="140mm" />
    <schematicsheet name="edge" displayName="4 J400 lines" sheetIndex={3} sheetWidth="120mm" sheetHeight="240mm" />
    <schematicsheet name="notes" displayName="5 Status" sheetIndex={4} sheetWidth="200mm" sheetHeight="80mm" />

    <schematicsection name="picoSec" displayName="Raspberry Pi Pico" />
    <schematicsection name="lvcSec" displayName="Address and control" />
    <schematicsection name="hctSec" displayName="Data transceiver" />
    <schematicsection name="edgeSec" displayName="J400" />
    <schematicsection name="noteSec" displayName="Not verified" />

    <pinheader
      name="J_PICO"
      pinCount={40}
      pitch="2.54mm"
      gender="male"
      schSheetName="pico"
      schSectionName="picoSec"
      schWidth={0.96}
      schFacingDirection="right"
      schX={0}
      schY={0}
      pcbX={-60}
      pcbY={0}
      pcbRotation={90}
      pinLabels={picoPinLabels}
      pinAttributes={{
        GND3: { requiresGround: true },
        GND38: { requiresGround: true },
        AGND: { requiresGround: true },
        "3V3": { requiresPower: true },
      }}
    />
    <chip
      name="U_ADDR"
      manufacturerPartNumber="SN74LVC245AN"
      footprint="dip20"
      schSheetName="lvc"
      schSectionName="lvcSec"
      schX={-6}
      schWidth={3}
      pcbX={-8}
      pcbY={32}
      pinLabels={dip245PinLabels}
      schPinArrangement={dip245Arrangement}
      pinAttributes={{ VCC: { requiresPower: true }, GND: { requiresGround: true } }}
    />
    <chip
      name="U_AHI"
      manufacturerPartNumber="SN74LVC245AN"
      footprint="dip20"
      schSheetName="lvc"
      schSectionName="lvcSec"
      schX={0}
      schWidth={3}
      pcbX={-8}
      pcbY={0}
      pinLabels={dip245PinLabels}
      schPinArrangement={dip245Arrangement}
      pinAttributes={{ VCC: { requiresPower: true }, GND: { requiresGround: true } }}
    />
    <chip
      name="U_CTL"
      manufacturerPartNumber="SN74LVC245AN"
      footprint="dip20"
      schSheetName="lvc"
      schSectionName="lvcSec"
      schX={6}
      schWidth={3}
      pcbX={-8}
      pcbY={-32}
      pinLabels={dip245PinLabels}
      schPinArrangement={dip245Arrangement}
      pinAttributes={{ VCC: { requiresPower: true }, GND: { requiresGround: true } }}
    />
    <chip
      name="U_DATA"
      manufacturerPartNumber="SN74HCT245N"
      footprint="dip20"
      schSheetName="hct"
      schSectionName="hctSec"
      schWidth={3}
      pcbX={28}
      pcbY={8}
      pinLabels={dip245PinLabels}
      schPinArrangement={dip245Arrangement}
      pinAttributes={{ VCC: { requiresPower: true }, GND: { requiresGround: true } }}
    />
    <resistor name="R_OE" resistance="10k" footprint="axial" schSheetName="hct" schSectionName="hctSec" schX={4} schY={2.6} pcbX={48} pcbY={28} />
    <resistor name="R_BASE" resistance="10k" footprint="axial" schSheetName="hct" schSectionName="hctSec" schX={4} schY={1.2} pcbX={48} pcbY={16} />
    <resistor name="R_BASE_PD" resistance="100k" footprint="axial" schSheetName="hct" schSectionName="hctSec" schX={4} schY={0.2} pcbX={48} pcbY={4} />
    <diode name="D_VSYS" manufacturerPartNumber="1N5817" footprint="axial" schSheetName="pico" schSectionName="picoSec" schX={4} schY={1} pcbX={-30} pcbY={40} pcbRotation={180} />
    <transistor name="Q_OE" type="npn" footprint="to92" schSheetName="hct" schSectionName="hctSec" schX={6.2} schY={1.4} pcbX={40} pcbY={-20} />
    <capacitor name="C_ADDR" capacitance="100nF" footprint="axial" schSheetName="lvc" schSectionName="lvcSec" schX={-6} schY={3.4} pcbX={12} pcbY={46} />
    <capacitor name="C_AHI" capacitance="100nF" footprint="axial" schSheetName="lvc" schSectionName="lvcSec" schX={0} schY={3.4} pcbX={12} pcbY={14} />
    <capacitor name="C_CTL" capacitance="100nF" footprint="axial" schSheetName="lvc" schSectionName="lvcSec" schX={6} schY={3.4} pcbX={12} pcbY={-46} />
    <capacitor name="C_DATA" capacitance="100nF" footprint="axial" schSheetName="hct" schSectionName="hctSec" schX={-3.5} schY={2.4} pcbX={48} pcbY={-8} />
    <pinheader
      name="J1"
      pinCount={44}
      pitch="2.54mm"
      gender="female"
      schSheetName="edge"
      schSectionName="edgeSec"
      schWidth={1.2}
      schFacingDirection="right"
      schX={0}
      schY={0}
      pcbX={62}
      pcbY={0}
      pcbRotation={90}
      pinLabels={j400PinLabels}
    />

    {ties([
      ["J_PICO.GP8", "net.PICO_OEB"],
      ["J_PICO.GP9", "net.PICO_DATA_nOE"],
      ["J_PICO.GP26", "net.PICO_A13"],
      ["J_PICO.GP27", "net.PICO_A14"],
      ["J_PICO.GP28", "net.PICO_A15"],
      ["J_PICO.RUN", "net.PICO_RUN_OPEN"],
      ["J_PICO.ADC_VREF", "net.PICO_ADC_VREF_OPEN"],
      ["J_PICO.3V3", "net.PICO_3V3"],
      ["J_PICO.3V3_EN", "net.PICO_3V3_EN_OPEN"],
      ["J_PICO.VSYS", "net.PICO_VSYS"],
      ["J_PICO.VBUS", "net.PICO_VBUS_USB_ONLY"],
      ["J_PICO.GND3", "net.GND"],
      ["J_PICO.GND8", "net.GND"],
      ["J_PICO.GND13", "net.GND"],
      ["J_PICO.GND18", "net.GND"],
      ["J_PICO.GND23", "net.GND"],
      ["J_PICO.GND28", "net.GND"],
      ["J_PICO.GND38", "net.GND"],
      ["J_PICO.AGND", "net.GND"],
    ])}
    {ties(lo.map((bit) => [`J_PICO.GP${bit}`, `net.PICO_D${bit}`] as const))}
    {ties(lo.map((bit) => [`J_PICO.GP${10 + bit}`, `net.PICO_A${bit}`] as const))}
    {ties([
      ["J_PICO.GP18", "net.PICO_A8"],
      ["J_PICO.GP19", "net.PICO_A9"],
      ["J_PICO.GP20", "net.PICO_A10"],
      ["J_PICO.GP21", "net.PICO_A11"],
      ["J_PICO.GP22", "net.PICO_A12"],
    ])}

    {ties(lo.map((bit) => [`U_ADDR.A${bit + 1}`, `net.PICO_A${bit}`] as const))}
    {ties(lo.map((bit) => [`U_ADDR.B${bit + 1}`, `net.J400_A${bit}`] as const))}
    {ties(lo.map((bit) => [`J1.A${bit}`, `net.J400_A${bit}`] as const))}
    {ties([
      ["U_ADDR.DIR", "net.GND"],
      ["U_ADDR.nOE", "net.GND"],
      ["U_ADDR.VCC", "net.PICO_3V3"],
      ["U_ADDR.GND", "net.GND"],
      ["C_ADDR.pin1", "net.PICO_3V3"],
      ["C_ADDR.pin2", "net.GND"],
    ])}

    {ties(hiBits.map((bit, i) => [`U_AHI.A${i + 1}`, `net.PICO_A${bit}`] as const))}
    {ties(hiBits.map((bit, i) => [`U_AHI.B${i + 1}`, `net.J400_A${bit}`] as const))}
    {ties(hiBits.map((bit) => [`J1.A${bit}`, `net.J400_A${bit}`] as const))}
    {ties([
      ["U_AHI.DIR", "net.GND"],
      ["U_AHI.nOE", "net.GND"],
      ["U_AHI.VCC", "net.PICO_3V3"],
      ["U_AHI.GND", "net.GND"],
      ["C_AHI.pin1", "net.PICO_3V3"],
      ["C_AHI.pin2", "net.GND"],
    ])}

    {ties([
      ["U_CTL.A1", "net.PICO_OEB"],
      ["U_CTL.B1", "net.J400_OEB"],
      ["J1.OEB", "net.J400_OEB"],
      ["U_CTL.A2", "net.WRB_NO_FREE_GPIO"],
      ["U_CTL.B2", "net.J400_WRB"],
      ["J1.WRB", "net.J400_WRB"],
      ["U_CTL.B3", "net.GND"],
      ["U_CTL.B4", "net.GND"],
      ["U_CTL.B5", "net.GND"],
      ["U_CTL.B6", "net.GND"],
      ["U_CTL.B7", "net.GND"],
      ["U_CTL.B8", "net.GND"],
      ["U_CTL.A3", "net.LVC_SPARE_3"],
      ["U_CTL.A4", "net.LVC_SPARE_4"],
      ["U_CTL.A5", "net.LVC_SPARE_5"],
      ["U_CTL.A6", "net.LVC_SPARE_6"],
      ["U_CTL.A7", "net.LVC_SPARE_7"],
      ["U_CTL.A8", "net.LVC_SPARE_8"],
      ["U_CTL.DIR", "net.GND"],
      ["U_CTL.nOE", "net.GND"],
      ["U_CTL.VCC", "net.PICO_3V3"],
      ["U_CTL.GND", "net.GND"],
      ["C_CTL.pin1", "net.PICO_3V3"],
      ["C_CTL.pin2", "net.GND"],
    ])}

    {ties(lo.map((bit) => [`U_DATA.A${bit + 1}`, `net.PICO_D${bit}`] as const))}
    {ties(lo.map((bit) => [`U_DATA.B${bit + 1}`, `net.J400_D${bit}`] as const))}
    {ties(lo.map((bit) => [`J1.D${bit}`, `net.J400_D${bit}`] as const))}
    {ties([
      ["U_DATA.DIR", "net.VSW_ASSUMED_5V"],
      ["U_DATA.VCC", "net.VSW_ASSUMED_5V"],
      ["J1.VSW", "net.VSW_ASSUMED_5V"],
      ["C_DATA.pin1", "net.VSW_ASSUMED_5V"],
      ["C_DATA.pin2", "net.GND"],
      ["D_VSYS.pin1", "net.VSW_ASSUMED_5V"],
      ["D_VSYS.pin2", "net.PICO_VSYS"],
      ["U_DATA.nOE", "net.HCT_nOE"],
      ["R_OE.pin1", "net.HCT_nOE"],
      ["R_OE.pin2", "net.VSW_ASSUMED_5V"],
      ["Q_OE.collector", "net.HCT_nOE"],
      ["Q_OE.emitter", "net.GND"],
      ["Q_OE.base", "net.Q_OE_BASE"],
      ["R_BASE.pin2", "net.Q_OE_BASE"],
      ["R_BASE.pin1", "net.PICO_DATA_nOE"],
      ["R_BASE_PD.pin1", "net.Q_OE_BASE"],
      ["R_BASE_PD.pin2", "net.GND"],
      ["U_DATA.GND", "net.GND"],
    ])}

    {ties([
      ["J1.GND_1", "net.GND"],
      ["J1.GND_43", "net.GND"],
      ["J1.VBAT", "net.J400_VBAT_NOT_LOGIC"],
    ])}
    {ties(idle.map((name) => [`J1.${name}`, `net.J400_${name}_UNUSED`] as const))}

    <schematictext schSheetName="pico" schSectionName="picoSec" schX={0} schY={5.0} anchor="center" color="#9b1c1c" fontSize={0.24} text="RASPBERRY PI PICO. PROPOSED GPIO MAP. NOT P28 FIRMWARE." />
    <schematictext schSheetName="lvc" schSectionName="lvcSec" schX={0} schY={4.4} anchor="center" color="#9b1c1c" fontSize={0.22} text="CANDIDATE SN74LVC245AN PDIP. STOCK UNCONFIRMED. VCC IS PICO 3V3." />
    <schematictext schSheetName="lvc" schSectionName="lvcSec" schX={0} schY={3.8} anchor="center" fontSize={0.16} text="DIR low: J400 B side toward Pico. WRB is buffered and stops. It has no free GPIO." />
    <schematictext schSheetName="hct" schSectionName="hctSec" schX={0} schY={3.5} anchor="center" color="#9b1c1c" fontSize={0.22} text="CANDIDATE SN74HCT245N PDIP. STOCK UNCONFIRMED." />
    <schematictext schSheetName="hct" schSectionName="hctSec" schX={0} schY={4.4} anchor="center" fontSize={0.16} text="VCC, DIR, and OE pull-up are assumed VSW 5 V. GP9 NPN holds Hi-Z when off." />
    <schematictext schSheetName="edge" schSectionName="edgeSec" schX={0} schY={6.3} anchor="center" color="#9b1c1c" fontSize={0.22} text="J400 LINE NUMBERS. GUIDE SAYS VERIFY. NOT PHOTO SILK ORDER." />
    <schematictext schSheetName="edge" schSectionName="edgeSec" schX={0} schY={5.7} anchor="center" fontSize={0.16} text="VSW assumed nominal 5 V. VBAT is not connected. Silk order is still unknown." />
    <schematictext schSheetName="notes" schSectionName="noteSec" schX={0} schY={3.6} anchor="center" color="#9b1c1c" fontSize={0.24} text="PRELIMINARY. NOT A POWER OR FAB RELEASE." />
    <schematictext schSheetName="notes" schSectionName="noteSec" schX={0} schY={2.8} anchor="center" fontSize={0.16} text="74HC245 is not used. LVC at 3.3 V accepts 5.5 V in and cannot drive a 5 V bus." />
    <schematictext schSheetName="notes" schSectionName="noteSec" schX={0} schY={2.1} anchor="center" fontSize={0.16} text="HCT at 4.5-5.5 V outputs a 5 V high. Those outputs stay on the J400 data pins." />
    <schematictext schSheetName="notes" schSectionName="noteSec" schX={0} schY={1.4} anchor="center" fontSize={0.16} text="ASSUMED VSW = 5 V. Verify 4.5-5.5 V for the HCT and <= 5.5 V at VSYS, spikes included, before power." />
    <schematictext schSheetName="notes" schSectionName="noteSec" schX={0} schY={0.7} anchor="center" fontSize={0.16} text="1N5817 anode on VSW, cathode on VSYS. Do not short VSYS to VBUS. No 7805. VBAT open." />
    <schematictext schSheetName="notes" schSectionName="noteSec" schX={0} schY={0.0} anchor="center" fontSize={0.16} text="Audio, SA0-3, DOUT, DREQ, CT, ASTRB, F2, WEB, BIGSPACEB, MNMIB, RESETSYSB stay off the Pico." />
    <schematictext schSheetName="notes" schSectionName="noteSec" schX={0} schY={-0.7} anchor="center" fontSize={0.16} text="P28 sheet in this folder is a reference module, not this Pico board. Issues stay open." />
    <pcbnotetext pcbX={0} pcbY={68} text="PLACEHOLDER FOOTPRINTS. NOT A CARTRIDGE PCB." />
  </board>
)
