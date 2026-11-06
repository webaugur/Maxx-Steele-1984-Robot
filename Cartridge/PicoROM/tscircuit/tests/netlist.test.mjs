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

const jedec = {
  VPP: 1,
  A16: 2,
  A15: 3,
  A12: 4,
  A7: 5,
  A6: 6,
  A5: 7,
  A4: 8,
  A3: 9,
  A2: 10,
  A1: 11,
  A0: 12,
  D0: 13,
  D1: 14,
  D2: 15,
  GND: 16,
  D3: 17,
  D4: 18,
  D5: 19,
  D6: 20,
  D7: 21,
  nCE: 22,
  A10: 23,
  nOE: 24,
  A11: 25,
  A9: 26,
  A8: 27,
  A13: 28,
  A14: 29,
  A17: 30,
  PGM: 31,
  VCC: 32,
}

const exportNetlist = () => {
  const dir = mkdtempSync(join(tmpdir(), "picorom-p32-"))
  const out = join(dir, "net.txt")
  execFileSync(bun, [
    tsci,
    "export",
    "picorom-p32-carrier.circuit.tsx",
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

test("P32 carrier pin numbers and J400 endpoints", () => {
  const netlist = exportNetlist()
  const committed = readFileSync(
    join(root, "..", "render", "picorom-p32-carrier.net.txt"),
    "utf8",
  )
  assert.equal(netlist.trim(), committed.trim())
  assert.match(netlist, /U_P32: PicoROM-P32-rev1\.10/)
  assert.doesNotMatch(netlist, /74HC|74HCT|74LVC|74LS|74AHCT|SN74|7805|LM340|PicoROM-P28|Pico\b/)

  const pins = pinRecords(netlist)
  const p32 = pins.filter((pin) => pin.ref === "U_P32")
  const j1 = pins.filter((pin) => pin.ref === "J1")
  assert.equal(p32.length, 32)
  assert.equal(j1.length, 44)

  for (const pin of p32) {
    assert.equal(pin.number, jedec[pin.name], `${pin.name} pin number`)
    assert.equal(pin.nets.length, 1)
  }

  const shares = (left, right) => left.nets.some((net) => right.nets.includes(net))
  const find = (list, name) => {
    const pin = list.find((item) => item.name === name)
    assert.ok(pin, name)
    return pin
  }

  let busEndpoints = 0
  for (let bit = 0; bit < 16; bit++) {
    const name = `A${bit}`
    const u = find(p32, name)
    const j = find(j1, name)
    assert.equal(shares(u, j), true, name)
    busEndpoints += 2
  }
  for (let bit = 0; bit < 8; bit++) {
    const name = `D${bit}`
    const u = find(p32, name)
    const j = find(j1, name)
    assert.equal(shares(u, j), true, name)
    busEndpoints += 2
  }
  assert.equal(shares(find(p32, "nOE"), find(j1, "OEB")), true)
  assert.equal(find(j1, "OEB").number, 12)
  assert.equal(shares(find(p32, "VCC"), find(j1, "VSW")), true)
  assert.equal(find(j1, "VSW").number, 41)

  const gndPins = ["nCE", "A16", "A17", "GND"].map((name) => find(p32, name))
  gndPins.push(find(j1, "GND_1"), find(j1, "GND_43"))
  for (const pin of gndPins) assert.equal(pin.nets[0], "GND", pin.name)

  assert.deepEqual(find(p32, "VPP").nets, ["NC_VPP"])
  assert.deepEqual(find(p32, "PGM").nets, ["NC_PGM"])
  assert.deepEqual(find(j1, "VBAT").nets, ["J400_VBAT_NOT_LOGIC"])
  assert.equal(find(j1, "VBAT").number, 39)
  assert.deepEqual(find(j1, "WRB").nets, ["NC_WRB"])

  const moduleNets = new Set(p32.flatMap((pin) => pin.nets))
  const ncJ = j1.filter((pin) => pin.nets.every((net) => !moduleNets.has(net)))
  assert.equal(ncJ.length, 16)
  assert.equal(busEndpoints, 48)
  console.log(
    `endpoints bus=${busEndpoints} oe=2 power=2 ground=${gndPins.length} module_nc=2 j1_nc=${ncJ.length} p32=${p32.length} j1=${j1.length}`,
  )
})
