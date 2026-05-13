# Gaussian AI Input Editor MVP Spec

## Overview

Desktop application for creating Gaussian input files using:

* structured UI
* molecule visualization
* AI-assisted command generation

This is NOT a general chemistry platform.

This MVP focuses ONLY on:

1. molecular structure handling
2. calculation specification editing
3. Gaussian input generation
4. AI-assisted editing

Execution, log parsing, and workflow management are OUT OF SCOPE.

---

# Core Design Principles

## 1. Structured State, Not Text Editing

The application MUST NOT treat Gaussian input as editable plain text.

Internal state must use structured domain models.

Gaussian input text is a rendered projection of internal state.

---

## 2. AI Generates Commands, Not Direct Mutations

AI must never directly mutate application state.

AI produces semantic commands.

Application reducer applies validated commands.

---

## 3. Shared Semantic Context

GUI and AI share semantic state:

* selected atoms
* active structure
* current calculation spec

AI must NOT access raw React component state or DOM.

---

# Tech Stack

## Frontend

* [React](https://react.dev?utm_source=chatgpt.com)
* TypeScript
* Zustand
* Vite

## Desktop

* [Tauri](https://tauri.app?utm_source=chatgpt.com)

## Molecule Visualization

* [3Dmol.js](https://3dmol.csb.pitt.edu?utm_source=chatgpt.com)

## Backend

* Rust

---

# Project Scope

## Included

* molecule import
* atom selection
* structured calculation editor
* Gaussian input rendering
* AI assistant panel
* screenshot export for AI context

## Excluded

* Gaussian execution
* job queue
* log parsing
* orbital visualization
* workflow graph
* conformer search
* multi-job projects

---

# Architecture

```text
Frontend UI
    ↓
App State
    ├ Domain State
    └ UI State
    ↓
Renderer
    ↓
Gaussian Input Text
```

AI flow:

```text
Natural Language
    ↓
AI Context Builder
    ↓
LLM
    ↓
Command List
    ↓
Reducer
    ↓
Updated State
```

---

# State Model

## AppState

```ts
type AppState = {
  domain: DomainState
  ui: UIState
}
```

---

# Domain State

```ts
type DomainState = {
  chemicalSpec: ChemicalSpec
}
```

---

# ChemicalSpec

```ts
type ChemicalSpec = {
  molecule: Molecule
  calculation: CalculationSpec
}
```

---

# Molecule

```ts
type Molecule = {
  atoms: Atom[]
  bonds: Bond[]
}
```

---

# Atom

```ts
type Atom = {
  id: number
  element: string
  position: [number, number, number]
}
```

---

# CalculationSpec

```ts
type CalculationSpec = {
  jobType: JobType
  method: Method
  basis: Basis
  solvent?: Solvent
  charge: number
  multiplicity: number
}
```

---

# UI State

```ts
type UIState = {
  selectedAtoms: number[]
}
```

Only semantic UI state should exist here.

DO NOT store React component internals.

---

# Renderer

Renderer converts ChemicalSpec into Gaussian input text.

```ts
renderGaussian(spec: ChemicalSpec): string
```

Gaussian text is derived state.

It must never be the source of truth.

---

# Command System

## Goal

All mutations must pass through semantic commands.

---

# Command Type

```ts
type Command =
  | { type: "SET_METHOD", method: Method }
  | { type: "SET_BASIS", basis: Basis }
  | { type: "SET_JOB_TYPE", jobType: JobType }
  | { type: "SET_SOLVENT", solvent: Solvent }
  | { type: "SET_CHARGE", charge: number }
  | { type: "SET_MULTIPLICITY", multiplicity: number }
```

---

# Reducer

```ts
reduce(state: AppState, command: Command): AppState
```

Reducer must be pure.

---

# AI Integration

## AI Responsibilities

AI may:

* interpret natural language
* propose commands
* explain calculation settings

AI may NOT:

* directly edit state
* edit Gaussian text
* mutate UI
* access DOM

---

# AI Context

AI receives compressed semantic context.

---

# AIContext

```ts
type AIContext = {
  selectedAtoms: AtomSummary[]
  calculation: CalculationSummary
  screenshot?: string
}
```

---

# Screenshot Support

Application should support viewport screenshot export.

Purpose:

* spatial understanding
* selection awareness
* visual grounding

Screenshot is supplementary context only.

Structured state remains primary.

---

# AI Output Format

LLM output MUST be structured JSON.

Example:

```json
{
  "commands": [
    {
      "type": "SET_METHOD",
      "method": "WB97XD"
    },
    {
      "type": "SET_SOLVENT",
      "solvent": "THF"
    }
  ],
  "explanation": "Updated method and solvent."
}
```

---

# Supported Initial Features

## Molecule Import

* xyz
* mol

---

# Selection

User can:

* select atom
* multi-select atoms

---

# Calculation Editing

Supported job types:

* opt
* freq
* opt+freq
* ts

Supported methods:

* B3LYP
* WB97XD

Supported basis sets:

* 6-31G(d)
* def2-SVP
* def2-TZVP

Supported solvents:

* THF
* Water

---

# UI Layout

```text
+-------------------+-------------------+
|                   |                   |
| Molecule Viewer   | Calculation Form  |
|                   |                   |
+-------------------+-------------------+
| Gaussian Input Preview                |
+---------------------------------------+
| AI Assistant                          |
+---------------------------------------+
```

---

# Validation Rules

Application must validate:

* multiplicity consistency
* required fields
* incompatible options

Validation belongs in domain layer.

NOT in AI layer.

---

# Future Extensibility

Architecture should support future:

* ORCA renderer
* Gaussian execution
* log parsing
* workflow DAG
* constraints
* AI diagnostics

without changing core state model.

---

# Important Constraints

## DO NOT

* use Gaussian text as application state
* let AI directly mutate state
* tightly couple renderer to UI
* embed chemistry logic in React components

## DO

* keep domain logic isolated
* use semantic commands
* keep renderer pure
* keep reducer pure

---

# MVP Success Criteria

Application is successful if user can:

1. import molecule
2. configure calculation
3. use AI to modify settings
4. generate valid Gaussian input
5. visually inspect structure
6. undo AI-generated changes

---

# Suggested Repository Structure

```text
src/
  domain/
    chemicalSpec.ts
    commands.ts
    reducer.ts
    renderer.ts

  ui/
    components/
    viewer/

  ai/
    contextBuilder.ts
    aiClient.ts

  app/
    store.ts
```

---

# Non-Goals

This application is NOT:

* a full computational chemistry suite
* a notebook system
* a workflow manager
* a general-purpose molecule editor

It is a structured Gaussian input IDE with AI assistance.

---

# Recommended First Milestone

Implement:

* molecule viewer
* ChemicalSpec state
* renderer
* command reducer
* manual form editing

BEFORE integrating LLM.

This minimizes debugging complexity.
