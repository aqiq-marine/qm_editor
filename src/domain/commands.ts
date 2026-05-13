import type { Basis, JobType, Method, Molecule, Solvent } from "./chemicalSpec";

export type Command =
  | { type: "SET_METHOD"; method: Method }
  | { type: "SET_BASIS"; basis: Basis }
  | { type: "SET_JOB_TYPE"; jobType: JobType }
  | { type: "SET_SOLVENT"; solvent?: Solvent }
  | { type: "SET_CHARGE"; charge: number }
  | { type: "SET_MULTIPLICITY"; multiplicity: number }
  | { type: "SET_MOLECULE"; molecule: Molecule }
  | { type: "TOGGLE_ATOM_SELECTION"; atomId: number }
  | { type: "CLEAR_SELECTION" };

export type AICommand = Exclude<
  Command,
  { type: "SET_MOLECULE" } | { type: "TOGGLE_ATOM_SELECTION" } | { type: "CLEAR_SELECTION" }
>;
