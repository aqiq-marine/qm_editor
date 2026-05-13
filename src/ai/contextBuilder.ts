import type { AppState } from "../domain/chemicalSpec";

export type AtomSummary = {
  id: number;
  element: string;
  position: [number, number, number];
};

export type CalculationSummary = {
  jobType: string;
  method: string;
  basis: string;
  solvent?: string;
  charge: number;
  multiplicity: number;
};

export type AIContext = {
  selectedAtoms: AtomSummary[];
  calculation: CalculationSummary;
  screenshot?: string;
};

export function buildAIContext(state: AppState, screenshot?: string): AIContext {
  const { molecule, calculation } = state.domain.chemicalSpec;
  const selectedAtoms = molecule.atoms
    .filter((atom) => state.ui.selectedAtoms.includes(atom.id))
    .map(({ id, element, position }) => ({ id, element, position }));

  return {
    selectedAtoms,
    calculation: { ...calculation },
    screenshot,
  };
}
