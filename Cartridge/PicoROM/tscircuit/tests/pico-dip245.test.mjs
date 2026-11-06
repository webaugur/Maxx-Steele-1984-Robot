import { execFileSync } from "node:child_process"
import { mkdtempSync, readFileSync } from "node:fs"
import { tmpdir } from "node:os"
import { test } from "node:test"
import assert from "node:assert/strict"
import { dirname, join } from "node:path"
import { fileURLToPath } from "node:url"

const root = join(dirname(fileURLToPath(import.meta.url)), "..")
const tsci = join(root, "node_modules", "tscircuit", "cli.mjs")
const bun = process.env.BUN ?? "bun"

const exportNetlist = () => {
  const dir = mkdtempSync(join(tmpdir(), "pico-dip-"))
  const out = join(dir, "net.txt")
  execFileSync(bun, [
    tsci, "export", "pico-dip245.circuit.tsx",
    "-f", "readable-netlist", "-o", out,
  ], { cwd: root, encoding: "utf8" })
  return readFileSync(out, "utf8")
}

const pinsOf = (netlist) => {
  const pins = []
  let component = ""
  for (const line of netlist.split("\n")) {
    const header = line.match(/^([A-Za-z0-9_]+)(?: \([^)]+\))?$/)
    if (header && !line.startsWith(" ") && !line.startsWith("-")) component = header[1]
    const match = line.match(/^- pin(\d+)\(([^)]+)\): NETS\(([^)]*)\)$/)
    if (match) {
      pins.push({
        component,
        number: Number(match[1]),
        name: match[2],
        nets: match[3].split(",").map((net) => net.trim()).filter(Boolean),
      })
    }
  }
  return pins
}

const net = (pins, component, name) => {
  const pin = pins.find((item) => item.component === component && item.name === name)
  assert.ok(pin, `${component}.${name} missing`)
  assert.equal(pin.nets.length, 1, `${component}.${name} nets ${pin.nets}`)
  return pin.nets[0]
}

const netNumber = (pins, component, number) => {
  const pin = pins.find((item) => item.component === component && item.number === number)
  assert.ok(pin, `${component} pin ${number} missing`)
  assert.equal(pin.nets.length, 1)
  return pin.nets[0]
}

test("J400 lines meet the Pico only through the DIP 245s", () => {
  const netlist = exportNetlist()
  const committed = readFileSync(join(root, "..", "render", "pico-dip245.net.txt"), "utf8")
  assert.equal(netlist.trim(), committed.trim())

  assert.match(netlist, /U_ADDR: SN74LVC245AN/)
  assert.match(netlist, /U_DATA: SN74HCT245N/)
  assert.doesNotMatch(netlist, /74HC245|74LS245|74LVC8T|74AHCT|PicoROM-P28/)

  const pins = pinsOf(netlist)
  const same = (a, b) => assert.equal(a, b)

  for (let bit = 0; bit < 8; bit += 1) {
    same(net(pins, "J1", `A${bit}`), net(pins, "U_ADDR", `B${bit + 1}`))
    same(net(pins, "U_ADDR", `A${bit + 1}`), net(pins, "J_PICO", `GP${10 + bit}`))
    same(net(pins, "J1", `D${bit}`), net(pins, "U_DATA", `B${bit + 1}`))
    same(net(pins, "U_DATA", `A${bit + 1}`), net(pins, "J_PICO", `GP${bit}`))
  }
  const hiPico = ["GP18", "GP19", "GP20", "GP21", "GP22", "GP26", "GP27", "GP28"]
  for (let index = 0; index < 8; index += 1) {
    const bit = index + 8
    same(net(pins, "J1", `A${bit}`), net(pins, "U_AHI", `B${index + 1}`))
    same(net(pins, "U_AHI", `A${index + 1}`), net(pins, "J_PICO", hiPico[index]))
  }

  same(net(pins, "J1", "OEB"), net(pins, "U_CTL", "B1"))
  same(net(pins, "U_CTL", "A1"), net(pins, "J_PICO", "GP8"))
  assert.notEqual(net(pins, "J1", "OEB"), net(pins, "U_DATA", "nOE"))

  same(net(pins, "J1", "WRB"), net(pins, "U_CTL", "B2"))
  const wrbOut = net(pins, "U_CTL", "A2")
  assert.equal(wrbOut, "WRB_NO_FREE_GPIO")
  assert.equal(pins.filter((pin) => pin.nets.includes(wrbOut)).length, 1)

  const vsw = net(pins, "J1", "VSW")
  assert.equal(vsw, "VSW_ASSUMED_5V")
  same(vsw, net(pins, "U_DATA", "VCC"))
  same(vsw, net(pins, "U_DATA", "DIR"))
  same(netNumber(pins, "R_OE", 2), vsw)
  same(netNumber(pins, "C_DATA", 1), vsw)
  same(netNumber(pins, "D_VSYS", 1), vsw)
  assert.equal(netNumber(pins, "D_VSYS", 2), "PICO_VSYS")
  assert.equal(net(pins, "J_PICO", "VSYS"), "PICO_VSYS")
  assert.notEqual(net(pins, "J_PICO", "VSYS"), vsw)
  assert.equal(net(pins, "J_PICO", "VBUS"), "PICO_VBUS_USB_ONLY")
  assert.notEqual(vsw, net(pins, "J1", "VBAT"))
  assert.doesNotMatch(netlist, /7805|LM340|LM317/)
  assert.equal(net(pins, "J1", "VBAT"), "J400_VBAT_NOT_LOGIC")
  assert.equal(pins.filter((pin) => pin.nets.includes("J400_VBAT_NOT_LOGIC")).length, 1)

  same(net(pins, "J1", "GND_1"), "GND")
  same(net(pins, "J1", "GND_43"), "GND")
  for (const ground of ["GND3", "GND8", "GND13", "GND18", "GND23", "GND28", "GND38", "AGND"]) {
    same(net(pins, "J_PICO", ground), "GND")
  }
  same(net(pins, "U_ADDR", "VCC"), net(pins, "J_PICO", "3V3"))
  assert.notEqual(net(pins, "U_ADDR", "VCC"), vsw)
  same(net(pins, "U_ADDR", "DIR"), "GND")
  const hctOe = net(pins, "U_DATA", "nOE")
  assert.equal(hctOe, "HCT_nOE")
  same(netNumber(pins, "R_OE", 1), hctOe)
  same(netNumber(pins, "R_OE", 2), vsw)
  same(netNumber(pins, "Q_OE", 1), hctOe)
  same(netNumber(pins, "Q_OE", 3), "GND")
  const base = netNumber(pins, "Q_OE", 2)
  same(netNumber(pins, "R_BASE", 2), base)
  same(netNumber(pins, "R_BASE_PD", 1), base)
  same(netNumber(pins, "R_BASE_PD", 2), "GND")
  same(netNumber(pins, "R_BASE", 1), net(pins, "J_PICO", "GP9"))
  assert.notEqual(net(pins, "J_PICO", "GP9"), hctOe)
  assert.notEqual(net(pins, "J_PICO", "GP9"), vsw)

  for (const name of ["AUDIO_IN", "F2", "WEB", "BIGSPACEB", "MNMIB", "RESETSYSB", "DOUT", "DREQ", "ASTRB"]) {
    const idle = net(pins, "J1", name)
    assert.match(idle, /_UNUSED$/)
    assert.equal(pins.filter((pin) => pin.nets.includes(idle) && pin.component === "J_PICO").length, 0)
  }

  const a10 = pins.find((pin) => pin.component === "J1" && pin.number === 16)
  assert.equal(a10.name, "A10")
  const oeb = pins.find((pin) => pin.component === "J1" && pin.number === 12)
  assert.equal(oeb.name, "OEB")
  const wrb = pins.find((pin) => pin.component === "J1" && pin.number === 14)
  assert.equal(wrb.name, "WRB")
})
