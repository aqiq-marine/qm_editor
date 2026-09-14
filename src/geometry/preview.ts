import type { Molecule } from "../bindings";

export type Vec3 = [number, number, number];
export type Quaternion = [number, number, number, number];

export type GeometryPreviewPlan = {
  movingAtomIds: number[];
  pivot: Vec3;
  axis: Vec3 | null;
  initialValue: number | null;
};

const sub = (a: Vec3, b: Vec3): Vec3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const add = (a: Vec3, b: Vec3): Vec3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
const scale = (a: Vec3, value: number): Vec3 => [a[0] * value, a[1] * value, a[2] * value];
const cross = (a: Vec3, b: Vec3): Vec3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const norm = (a: Vec3) => Math.hypot(a[0], a[1], a[2]);

export function normalize(value: Vec3): Vec3 | null {
  const length = norm(value);
  return length > 1e-12 ? scale(value, 1 / length) : null;
}

export function quaternionFromAxisAngle(axis: Vec3, angle: number): Quaternion {
  const unit = normalize(axis);
  if (!unit) return [1, 0, 0, 0];
  const half = angle / 2;
  const s = Math.sin(half);
  return [Math.cos(half), unit[0] * s, unit[1] * s, unit[2] * s];
}

function rotate(value: Vec3, quaternion: Quaternion): Vec3 {
  const [, x, y, z] = quaternion;
  const w = quaternion[0];
  const qv: Vec3 = [x, y, z];
  const t = scale(cross(qv, value), 2);
  return add(value, add(scale(t, w), cross(qv, t)));
}

export function applyRigidPreview(molecule: Molecule, plan: GeometryPreviewPlan, targetValue: number): Molecule {
  const delta = (targetValue - (plan.initialValue ?? targetValue)) * Math.PI / 180;
  const quaternion = plan.axis ? quaternionFromAxisAngle(plan.axis, delta) : [1, 0, 0, 0] as Quaternion;
  const moving = new Set(plan.movingAtomIds);
  return {
    ...molecule,
    atoms: molecule.atoms.map((atom) => {
      if (!moving.has(atom.id)) return atom;
      const relative = sub(atom.position, plan.pivot);
      return { ...atom, position: add(plan.pivot, rotate(relative, quaternion)) };
    }),
  };
}

export function dihedralPreviewPlan(molecule: Molecule, atomIds: [number, number, number, number], initialValue: number): GeometryPreviewPlan | null {
  const positions = atomIds.map((id) => molecule.atoms.find((atom) => atom.id === id)?.position as Vec3 | undefined);
  if (positions.some((position) => !position)) return null;
  const axis = normalize(sub(positions[2]!, positions[1]!));
  if (!axis) return null;
  const blocked = new Set([atomIds[1], atomIds[2]]);
  const moving = new Set<number>([atomIds[3]]);
  const queue = [atomIds[3]];
  while (queue.length > 0) {
    const current = queue.shift()!;
    for (const bond of molecule.bonds) {
      if (!bond.atomIds.includes(current)) continue;
      const neighbor = bond.atomIds[0] === current ? bond.atomIds[1] : bond.atomIds[0];
      if (blocked.has(neighbor) || moving.has(neighbor)) continue;
      moving.add(neighbor);
      queue.push(neighbor);
    }
  }
  return { movingAtomIds: [...moving], pivot: positions[2]!, axis, initialValue };
}
