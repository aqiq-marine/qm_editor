import { useEffect, useRef, useState } from "react";
import { createViewer, type AtomSpec, type GLViewer } from "3dmol";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useAppStore } from "./app/store";
import { buildAIContext } from "./ai/contextBuilder";
import { proposeCommands, type AIResult } from "./ai/aiClient";
import {
  supportedBases,
  supportedJobTypes,
  supportedMethods,
  supportedSolvents,
  type Molecule,
  type Solvent,
  type ValidationMessage,
} from "./domain/chemicalSpec";

function App() {
  return <EditorShell />;
}

function EditorShell() {
  const { state, loadInitialState } = useAppStore();
  const [gaussian, setGaussian] = useState("");
  const [messages, setMessages] = useState<ValidationMessage[]>([]);

  useEffect(() => {
    void loadInitialState();
  }, [loadInitialState]);

  useEffect(() => {
    if (!state) return;
    const spec = state.domain.chemicalSpec;
    void invoke<string>("render_gaussian", { spec }).then(setGaussian);
    void invoke<ValidationMessage[]>("validate_chemical_spec", { spec }).then(setMessages);
  }, [state]);

  if (!state) {
    return (
      <main className="app-shell">
        <section className="viewer-panel">Loading editor...</section>
      </main>
    );
  }

  const spec = state.domain.chemicalSpec;

  return (
    <main className="app-shell">
      <header className="topbar">
        <div>
          <p className="eyebrow">Gaussian Input IDE</p>
          <h1>DFT Input File Editor</h1>
        </div>
        <ImportControl />
      </header>

      <section className="workspace">
        <MoleculeViewer />
        <CalculationForm messages={messages} />
      </section>

      <section className="preview-panel" aria-label="Gaussian input preview">
        <div className="panel-heading">
          <h2>Gaussian Input Preview</h2>
          <span>{spec.molecule.atoms.length} atoms</span>
        </div>
        <pre>{gaussian}</pre>
      </section>

      <AIAssistant />
    </main>
  );
}

function ImportControl() {
  const { dispatchCommand } = useAppStore();
  const [error, setError] = useState("");

  async function importFile(file: File | undefined) {
    if (!file) return;
    try {
      const text = await file.text();
      const molecule = await invoke<Molecule>("parse_molecule_file", { fileName: file.name, text });
      await dispatchCommand({ type: "SET_MOLECULE", molecule });
      setError("");
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : "Failed to import molecule.");
    }
  }

  return (
    <div className="import-control">
      <label className="file-button">
        Import XYZ/MOL
        <input
          type="file"
          accept=".xyz,.mol"
          onChange={(event) => void importFile(event.currentTarget.files?.[0])}
        />
      </label>
      {error ? <span className="inline-error">{error}</span> : null}
    </div>
  );
}

function MoleculeViewer() {
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
      viewerRef.current = createViewer(container, { backgroundColor: "#f8fafc" });
    }

    const viewer = viewerRef.current;
    viewer.removeAllModels();
    viewer.removeAllLabels();
    viewer.addModel(moleculeToXyz(molecule), "xyz");
    viewer.setStyle({}, { stick: { radius: 0.15 }, sphere: { scale: 0.34 } });
    for (const atomId of selected) {
      viewer.setStyle(
        { serial: atomId },
        { stick: { radius: 0.2, color: "#c27a22" }, sphere: { scale: 0.46, color: "#f4b13d" } },
      );
    }
    viewer.setClickable({}, true, (atom: AtomSpec) => {
      const atomId = atom.serial ?? (atom.index === undefined ? undefined : atom.index + 1);
      if (atomId !== undefined) void dispatchCommand({ type: "TOGGLE_ATOM_SELECTION", atomId });
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
          <p>{selected.length} selected</p>
        </div>
        <button type="button" onClick={() => void dispatchCommand({ type: "CLEAR_SELECTION" })}>
          Clear
        </button>
      </div>

      <div ref={containerRef} className="molecule-canvas" role="img" aria-label="3D molecule viewer" />
    </section>
  );
}

function CalculationForm({ messages }: { messages: ValidationMessage[] }) {
  const { state, dispatchCommand } = useAppStore();
  if (!state) return null;
  const calculation = state.domain.chemicalSpec.calculation;

  return (
    <section className="form-panel" aria-label="Calculation form">
      <div className="panel-heading">
        <h2>Calculation</h2>
      </div>

      <div className="form-grid">
        <SelectField
          label="Job type"
          value={calculation.jobType}
          options={supportedJobTypes}
          onChange={(jobType) => void dispatchCommand({ type: "SET_JOB_TYPE", jobType })}
        />
        <SelectField
          label="Method"
          value={calculation.method}
          options={supportedMethods}
          onChange={(method) => void dispatchCommand({ type: "SET_METHOD", method })}
        />
        <SelectField
          label="Basis"
          value={calculation.basis}
          options={supportedBases}
          onChange={(basis) => void dispatchCommand({ type: "SET_BASIS", basis })}
        />
        <label>
          Solvent
          <select
            value={calculation.solvent ?? ""}
            onChange={(event) =>
              void dispatchCommand({
                type: "SET_SOLVENT",
                solvent: event.currentTarget.value ? (event.currentTarget.value as Solvent) : undefined,
              })
            }
          >
            <option value="">Gas phase</option>
            {supportedSolvents.map((solvent) => (
              <option key={solvent} value={solvent}>
                {solvent}
              </option>
            ))}
          </select>
        </label>
        <NumberField
          label="Charge"
          value={calculation.charge}
          onChange={(charge) => void dispatchCommand({ type: "SET_CHARGE", charge })}
        />
        <NumberField
          label="Multiplicity"
          min={1}
          value={calculation.multiplicity}
          onChange={(multiplicity) => void dispatchCommand({ type: "SET_MULTIPLICITY", multiplicity })}
        />
      </div>

      <div className="validation-list">
        {messages.length === 0 ? (
          <p className="valid">Ready to render Gaussian input.</p>
        ) : (
          messages.map((message) => (
            <p key={message.message} className={message.level}>
              {message.message}
            </p>
          ))
        )}
      </div>
    </section>
  );
}

function SelectField<T extends string>({
  label,
  value,
  options,
  onChange,
}: {
  label: string;
  value: T;
  options: readonly T[];
  onChange: (value: T) => void;
}) {
  return (
    <label>
      {label}
      <select value={value} onChange={(event) => onChange(event.currentTarget.value as T)}>
        {options.map((option) => (
          <option key={option} value={option}>
            {option}
          </option>
        ))}
      </select>
    </label>
  );
}

function NumberField({
  label,
  value,
  min,
  onChange,
}: {
  label: string;
  value: number;
  min?: number;
  onChange: (value: number) => void;
}) {
  return (
    <label>
      {label}
      <input
        type="number"
        min={min}
        value={value}
        onChange={(event) => onChange(Number(event.currentTarget.value))}
      />
    </label>
  );
}

function AIAssistant() {
  const { state, applyCommands, undo, canUndo } = useAppStore();
  const [request, setRequest] = useState("");
  const [result, setResult] = useState<AIResult | null>(null);
  const [screenshot, setScreenshot] = useState<string | undefined>();

  function generateCommands() {
    if (!state) return;
    const aiContext = buildAIContext(state, screenshot);
    setResult(proposeCommands(request, aiContext));
  }

  function captureScreenshot() {
    const canvas = document.querySelector<HTMLCanvasElement>(".molecule-canvas canvas");
    setScreenshot(canvas?.toDataURL("image/png"));
  }

  function applyAICommands() {
    if (!result || result.commands.length === 0) return;
    void applyCommands(result.commands);
  }

  return (
    <section className="assistant-panel" aria-label="AI assistant">
      <div className="panel-heading">
        <h2>AI Assistant</h2>
        <button type="button" disabled={!canUndo()} onClick={undo}>
          Undo
        </button>
      </div>
      <textarea
        value={request}
        onChange={(event) => setRequest(event.currentTarget.value)}
        placeholder="Set WB97XD with def2-TZVP in THF, opt+freq, charge 0, multiplicity 1"
      />
      <div className="assistant-actions">
        <button type="button" onClick={captureScreenshot}>
          Capture View
        </button>
        <button type="button" onClick={generateCommands}>
          Generate Commands
        </button>
        <button type="button" disabled={!result || result.commands.length === 0} onClick={applyAICommands}>
          Apply Commands
        </button>
      </div>
      {result ? (
        <div className="ai-output">
          <p>
            {result.explanation}
            {screenshot ? " Screenshot context attached." : ""}
          </p>
          <pre>{JSON.stringify({ commands: result.commands, explanation: result.explanation }, null, 2)}</pre>
        </div>
      ) : null}
    </section>
  );
}

function moleculeToXyz(molecule: { name: string; atoms: { element: string; position: [number, number, number] }[] }) {
  return [
    String(molecule.atoms.length),
    molecule.name,
    ...molecule.atoms.map(({ element, position }) => `${element} ${position[0]} ${position[1]} ${position[2]}`),
  ].join("\n");
}

export default App;
