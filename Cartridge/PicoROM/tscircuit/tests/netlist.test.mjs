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

const parsePins = (netlist) => {
  const pins = []
  for (const line of netlist.split("\n")) {
    const match = line.match(
      /^- pin(\d+)\(([^)]+)\): NETS\(([^)]*)\)$/,
    )
    if (match) {
      pins.push({
        number: Number(match[1]),
        name: match[2],
        nets: match[3].split(",").map((net) => net.trim()).filter(Boolean),
      })
    }
  }
  return pins
}

test("carrier netlist keeps every J1 finger off the P28 module", () => {
  const netlist = exportNetlist()
  const committed = readFileSync(
    join(root, "..", "render", "picorom-carrier.net.txt"),
    "utf8",
  )
  assert.equal(netlist.trim(), committed.trim())

  assert.match(netlist, /U_P28: PicoROM-P28-rev1\.10/)
  assert.doesNotMatch(netlist, /74HC|74HCT|74LVC|74LS|74AHCT|RP2040|Pico\b/)

  const pins = parsePins(netlist)
  const p28 = pins.filter((pin) => pin.nets.some((net) => net.startsWith("P28_")))
  const j1 = pins.filter((pin) =>
    pin.nets.some((net) => net.startsWith("UNRESOLVED_J1_")),
  )
  assert.equal(p28.length, 28)
  assert.equal(j1.length, 44)

  const seen = new Map()
  for (const pin of [...p28, ...j1]) {
    assert.equal(pin.nets.length, 1, `${pin.name} must have one net`)
    const net = pin.nets[0]
    assert.equal(seen.has(net), false, `net ${net} is shared`)
    seen.set(net, pin.name)
  }

  for (const pin of j1) {
    assert.match(pin.nets[0], /^UNRESOLVED_J1_\d{2}$/)
    assert.equal(pin.name, `IDX${pin.nets[0].slice(-2)}`)
  }

  const byName = Object.fromEntries(p28.map((pin) => [pin.name, pin]))
  assert.equal(byName.VCC.number, 28)
  assert.equal(byName.GND.number, 14)
  assert.equal(byName.A15.number, 1)
  assert.equal(byName.A0.number, 10)
  assert.equal(byName.D0.number, 11)
  assert.equal(byName.D7.number, 19)
  assert.equal(byName.nCE.number, 20)
  assert.equal(byName.nOE.number, 22)
  assert.equal(byName.A14.number, 27)
  for (const pin of p28) assert.match(pin.nets[0], /_OPEN$/)
})
