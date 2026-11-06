/**
 * 44 drawing indexes for the cartridge edge.
 * The STEP model establishes 44 contacts at 2.54 mm, 22 per face.
 * No index is a confirmed signal. Silk numbers 1–22 on one photo face
 * are not copied onto these pins.
 */
const labels = Array.from({ length: 44 }, (_, index) => {
  const nn = String(index + 1).padStart(2, "0")
  return `IDX${nn}` as const
})

export const j1PinLabels = Object.fromEntries(
  labels.map((label, index) => [`pin${index + 1}`, label]),
) as { [pin: `pin${number}`]: string }

export const j1OpenNets = labels.map((label, index) => {
  const nn = String(index + 1).padStart(2, "0")
  return [label, `UNRESOLVED_J1_${nn}`] as const
})
