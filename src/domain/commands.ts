import type { Basis, Element, JobType, Method, Molecule, Solvent } from "./chemicalSpec";

export type Command =
  | { type: "SET_METHOD"; method: Method }
  | { type: "SET_BASIS"; basis: Basis }
  | { type: "SET_JOB_TYPE"; jobType: JobType }
  | { type: "SET_SOLVENT"; solvent?: Solvent }
  | { type: "SET_CHARGE"; charge: number }
  | { type: "SET_MULTIPLICITY"; multiplicity: number }
  | { type: "SET_BOND_LENGTH"; atomIds: [number, number]; length: number }
  | { type: "SET_BOND_ANGLE"; atomIds: [number, number, number]; angle: number }
  | { type: "SET_DIHEDRAL_ANGLE"; atomIds: [number, number, number, number]; angle: number }
  | {
      type: "ADD_ATOM";
      element: Element;
      position: [number, number, number];
      isotope?: number;
      nuclearSpin?: number;
    }
  | { type: "DELETE_ATOM"; atomId: number }
  | { type: "ADD_BOND"; atomIds: [number, number]; order: 1 | 2 | 3 }
  | { type: "DELETE_BOND"; bondId: number }
  | { type: "SET_MOLECULE"; molecule: Molecule }
  | { type: "TOGGLE_ATOM_SELECTION"; atomId: number }
  | { type: "CLEAR_SELECTION" };

export type AICommand = Exclude<
  Command,
  { type: "SET_MOLECULE" } | { type: "TOGGLE_ATOM_SELECTION" } | { type: "CLEAR_SELECTION" }
>;

export type AIResult = {
  commands: AICommand[];
  explanation: string;
};
