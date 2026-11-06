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
  const dir = mkdtempSync(join(tmpdir(), "picorom-net-"))
  const out = join(dir, "net.txt")
  execFileSync(bun, [
    tsci,
    "export",
    "picorom-carrier.circuit.tsx",
    "-f",
    "readable-netlist",
    "-o",
    out,
  ], { cwd: root, encoding: "utf8" })
  return readFileSync(out, "utf8")
}

const pinRecords = (netlist) => {
  const pins = []
  let ref = null
  let inPins = false
  for (const line of netlist.split("\n")) {
    if (line.startsWith("COMPONENT_PINS:")) {
      inPins = true
      continue
    }
    if (!inPins) continue
    const pin = line.match(/^- pin(\d+)\(([^)]+)\): NETS\(([^)]*)\)$/)
    if (pin) {
      pins.push({
        ref,
        number: Number(pin[1]),
        name: pin[2],
        nets: pin[3].split(",").map((net) => net.trim()).filter(Boolean),
      })
      continue
    }
    if (line.trim()) ref = line.split(" ")[0]
  }
  return pins
}

const members = (pins, net) =>
  pins
    .filter((pin) => pin.nets.includes(net))
    .map((pin) => `${pin.ref}.${pin.name}`)

test("P28 carrier uses the module pins and no external buffer", () => {
  const netlist = exportNetlist()
  const committed = readFileSync(
    join(root, "..", "render", "picorom-carrier.net.txt"),
    "utf8",
  )
  assert.equal(netlist.trim(), committed.trim())

  assert.match(netlist, /U_P28: PicoROM-P28-rev1\.10/)
  assert.doesNotMatch(netlist, /74HC|74HCT|74LVC|74LS|74AHCT|SN74|7805|LM340|Pico\b|RP2040/)

  const pins = pinRecords(netlist)
  const on = (net) => members(pins, net).sort()

  for (let bit = 0; bit < 16; bit++) {
    assert.deepEqual(on(`J400_A${bit}`), [`J1.A${bit}`, `U_P28.A${bit}`].sort())
  }
  for (let bit = 0; bit < 8; bit++) {
    assert.deepEqual(on(`J400_D${bit}`), [`J1.D${bit}`, `U_P28.D${bit}`].sort())
  }

  assert.deepEqual(on("J400_OEB"), ["J1.OEB", "U_P28.nOE"].sort())
  assert.ok(on("GND").includes("U_P28.nCE"))
  assert.ok(on("GND").includes("U_P28.GND"))
  assert.ok(on("GND").includes("J1.GND_1"))
  assert.ok(on("GND").includes("J1.GND_43"))
  assert.deepEqual(on("VSW_ASSUMED_5V"), ["J1.VSW", "U_P28.VCC"].sort())
  assert.deepEqual(on("J400_VBAT_NOT_LOGIC"), ["J1.VBAT"])
  assert.deepEqual(on("NC_WRB"), ["J1.WRB"])

  const p28 = pins.filter((pin) => pin.ref === "U_P28")
  const j1 = pins.filter((pin) => pin.ref === "J1")
  assert.equal(p28.length, 28)
  assert.equal(j1.length, 44)
  const shares = (left, right) =>
    left.nets.some((net) => right.nets.includes(net))
  let busEndpoints = 0
  for (const name of [
    ...Array.from({ length: 16 }, (_, bit) => `A${bit}`),
    ...Array.from({ length: 8 }, (_, bit) => `D${bit}`),
  ]) {
    const u = p28.find((pin) => pin.name === name)
    const j = j1.find((pin) => pin.name === name)
    assert.ok(u && j, name)
    assert.equal(shares(u, j), true, `${name} endpoints differ`)
    assert.equal(u.nets.length, 1)
    assert.equal(j.nets.length, 1)
    busEndpoints += 2
  }
  const nce = p28.find((pin) => pin.name === "nCE")
  const uGnd = p28.find((pin) => pin.name === "GND")
  const g1 = j1.find((pin) => pin.name === "GND_1")
  const g43 = j1.find((pin) => pin.name === "GND_43")
  assert.equal(shares(nce, uGnd) && shares(uGnd, g1) && shares(g1, g43), true)
  const vcc = p28.find((pin) => pin.name === "VCC")
  const vsw = j1.find((pin) => pin.name === "VSW")
  const noe = p28.find((pin) => pin.name === "nOE")
  const oeb = j1.find((pin) => pin.name === "OEB")
  assert.equal(shares(vcc, vsw), true)
  assert.equal(shares(noe, oeb), true)
  const moduleNets = new Set(p28.flatMap((pin) => pin.nets))
  const nc = j1.filter((pin) => pin.nets.every((net) => !moduleNets.has(net)))
  assert.equal(nc.length, 16)
  assert.equal(busEndpoints, 48)
  console.log(
    `endpoints bus=${busEndpoints} power=2 ground=4 oe=2 nc=${nc.length} p28=${p28.length} j1=${j1.length}`,
  )

  const numberOf = (ref, name) => {
    const pin = pins.find((item) => item.ref === ref && item.name === name)
    assert.ok(pin, `${ref} ${name}`)
    return pin.number
  }
  assert.equal(numberOf("U_P28", "A15"), 1)
  assert.equal(numberOf("U_P28", "A0"), 10)
  assert.equal(numberOf("U_P28", "D0"), 11)
  assert.equal(numberOf("U_P28", "GND"), 14)
  assert.equal(numberOf("U_P28", "nCE"), 20)
  assert.equal(numberOf("U_P28", "nOE"), 22)
  assert.equal(numberOf("U_P28", "A14"), 27)
  assert.equal(numberOf("U_P28", "VCC"), 28)
  assert.equal(numberOf("J1", "OEB"), 12)
  assert.equal(numberOf("J1", "VSW"), 41)
  assert.equal(numberOf("J1", "VBAT"), 39)
})
