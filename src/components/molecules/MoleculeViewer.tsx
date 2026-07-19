import { useEffect, useRef } from 'react';
import { createViewer, type AtomSpec, type GLViewer } from '3dmol';
import { useAppStore } from '../../app/store';
import { formatAtomLabel, formatSelectedDisplayAtoms, moleculeToMol } from '../../utils/moleculeHelpers';

export function MoleculeViewer() {
  const { state, dispatchCommand } = useAppStore();
  if (!state) return null;
  const { molecule } = state.domain.chemicalSpec;
  const selected = state.ui.selectedAtoms;
  const containerRef = useRef<HTMLDivElement | null>(null);
  const viewerRef = useRef<GLViewer | null>(null);

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    if (!viewerRef.current) {
      viewerRef.current = createViewer(container, { backgroundColor: '#f8fafc' });
    }

    const viewer = viewerRef.current;
    viewer.removeAllModels();
    viewer.removeAllLabels();
    viewer.addModel(moleculeToMol(molecule), 'mol');
    viewer.setStyle({}, { stick: { radius: 0.15 }, sphere: { scale: 0.34 } });

    molecule.atoms.forEach((atom, index) => {
      viewer.addLabel(formatAtomLabel(index + 1, atom.formalCharge ?? 0), {
        position: { x: atom.position[0], y: atom.position[1], z: atom.position[2] },
        backgroundColor: 'white',
        backgroundOpacity: 0.5,
        fontSize: 12,
        fontColor: 'black',
      });
    });

    for (const atomId of selected) {
      const atomIndex = molecule.atoms.findIndex((atom) => atom.id === atomId);
      if (atomIndex < 0) continue;
      viewer.setStyle(
        { index: atomIndex },
        { stick: { radius: 0.2, color: '#c27a22' }, sphere: { scale: 0.46, color: '#f4b13d' } },
      );
    }
    viewer.setClickable({}, true, (atom: AtomSpec) => {
      const atomId = atom.index === undefined ? undefined : molecule.atoms[atom.index]?.id;
      if (atomId !== undefined) void dispatchCommand({ type: 'TOGGLE_ATOM_SELECTION', atom_id: atomId });
    });
    viewer.zoomTo();
    viewer.render();
    viewer.resize();
  }, [dispatchCommand, molecule, selected]);

  return (
    <section className="viewer-panel" aria-label="Molecule viewer">
      <div className="panel-heading">
        <div>
          <h2>{molecule.name}</h2>
          <p>{formatSelectedDisplayAtoms(molecule, selected)}</p>
        </div>
        <button type="button" onClick={() => void dispatchCommand({ type: 'CLEAR_SELECTION' })}>
          Clear
        </button>
      </div>

      <div ref={containerRef} className="molecule-canvas" role="img" aria-label="3D molecule viewer" />
    </section>
  );
}
