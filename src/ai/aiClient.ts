import type { AIContext } from "./contextBuilder";
import type { AICommand } from "../domain/commands";
import {
  supportedBases,
  supportedJobTypes,
  supportedMethods,
  supportedSolvents,
  type Basis,
  type JobType,
  type Method,
  type Solvent,
} from "../domain/chemicalSpec";

export type AIResult = {
  commands: AICommand[];
  explanation: string;
};

export function proposeCommands(input: string, context: AIContext): AIResult {
  const trimmed = input.trim();
  if (!trimmed) {
    return { commands: [], explanation: "No request was provided." };
  }

  const jsonResult = parseJsonCommands(trimmed);
  if (jsonResult) return jsonResult;

  const commands: AICommand[] = [];
  const normalized = trimmed.toLowerCase();

  for (const method of supportedMethods) {
    if (normalized.includes(method.toLowerCase())) {
      commands.push({ type: "SET_METHOD", method });
    }
  }

  for (const basis of supportedBases) {
    if (normalized.includes(basis.toLowerCase())) {
      commands.push({ type: "SET_BASIS", basis });
    }
  }

  for (const solvent of supportedSolvents) {
    if (normalized.includes(solvent.toLowerCase())) {
      commands.push({ type: "SET_SOLVENT", solvent });
    }
  }

  if (/\bno\s+solvent\b|\bgas\s+phase\b/.test(normalized)) {
    commands.push({ type: "SET_SOLVENT", solvent: undefined });
  }

  const jobType = inferJobType(normalized);
  if (jobType) commands.push({ type: "SET_JOB_TYPE", jobType });

  const charge = normalized.match(/\bcharge\s*(-?\d+)\b/);
  if (charge) commands.push({ type: "SET_CHARGE", charge: Number(charge[1]) });

  const multiplicity = normalized.match(/\b(?:multiplicity|mult)\s*(\d+)\b/);
  if (multiplicity) {
    commands.push({ type: "SET_MULTIPLICITY", multiplicity: Number(multiplicity[1]) });
  }

  const uniqueCommands = dedupe(commands);
  return {
    commands: uniqueCommands,
    explanation:
      uniqueCommands.length > 0
        ? `Proposed ${uniqueCommands.length} command(s) from the request. Current method is ${context.calculation.method}.`
        : "No supported calculation changes were found. Try mentioning method, basis, job type, solvent, charge, or multiplicity.",
  };
}

function parseJsonCommands(text: string): AIResult | undefined {
  try {
    const parsed = JSON.parse(text) as Partial<AIResult>;
    if (!Array.isArray(parsed.commands)) return undefined;
    const commands = parsed.commands.filter(isAICommand);
    return {
      commands,
      explanation: typeof parsed.explanation === "string" ? parsed.explanation : "Parsed JSON commands.",
    };
  } catch {
    return undefined;
  }
}

function isAICommand(value: unknown): value is AICommand {
  if (!value || typeof value !== "object" || !("type" in value)) return false;
  const command = value as Record<string, unknown>;
  if (command.type === "SET_METHOD") return supportedMethods.includes(command.method as Method);
  if (command.type === "SET_BASIS") return supportedBases.includes(command.basis as Basis);
  if (command.type === "SET_JOB_TYPE") return supportedJobTypes.includes(command.jobType as JobType);
  if (command.type === "SET_SOLVENT") {
    return command.solvent === undefined || supportedSolvents.includes(command.solvent as Solvent);
  }
  if (command.type === "SET_CHARGE") return Number.isInteger(command.charge);
  if (command.type === "SET_MULTIPLICITY") return Number.isInteger(command.multiplicity);
  return false;
}

function inferJobType(text: string): JobType | undefined {
  if (/\bts\b|transition state/.test(text)) return "ts";
  if (/opt(?:imize|imization)?.*freq|freq.*opt(?:imize|imization)?/.test(text)) return "opt+freq";
  if (/freq|frequency/.test(text)) return "freq";
  if (/opt|optimize|optimization/.test(text)) return "opt";
  return undefined;
}

function dedupe(commands: AICommand[]): AICommand[] {
  const byType = new Map<string, AICommand>();
  for (const command of commands) byType.set(command.type, command);
  return Array.from(byType.values());
}
