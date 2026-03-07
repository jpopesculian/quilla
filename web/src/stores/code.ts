import { create } from "zustand";
import type { ParseItem } from "../../../crates/js/pkg/ParseItem";
import type { Operation } from "../../../crates/js/pkg/Operation";
import {
  parse,
  CircuitDrawing,
  Circuit,
} from "../../../crates/js/pkg/quilla_js.js";

export type { ParseItem, Operation };

function qbitsForOp(op: Operation): number[] {
  switch (op.kind) {
    case "cx":
    case "cy":
    case "cz":
      return [op.control, op.target];
    case "swap":
      return [op.first, op.second];
    case "meas":
      return [op.qbit];
    default:
      return [op.target];
  }
}

function circuitSize(parsed: ParseItem[]): { qbits: number; cbits: number } {
  let maxQbit = -1;
  let maxCbit = -1;
  for (const item of parsed) {
    if ("operation" in item) {
      for (const q of qbitsForOp(item.operation)) {
        if (q > maxQbit) maxQbit = q;
      }
      if (item.operation.kind === "meas" && item.operation.cbit > maxCbit) {
        maxCbit = item.operation.cbit;
      }
    }
  }
  return { qbits: maxQbit + 1, cbits: maxCbit + 1 };
}

function drawCircuit(
  parsed: ParseItem[],
  qbits: number,
  cbits: number,
): string {
  if (qbits === 0) return "";
  const drawing = new CircuitDrawing(qbits, cbits);
  for (const item of parsed) {
    if ("operation" in item) {
      drawing.draw(item.operation);
    }
  }
  const result = drawing.toString();
  drawing.free();
  return result;
}

interface CodeState {
  code: string;
  parsed: ParseItem[];
  qbits: number;
  cbits: number;
  drawing: string;
  samples: Map<string, number> | null;
  setCode: (code: string) => void;
}

function sampleCircuit(
  parsed: ParseItem[],
  qbits: number,
  cbits: number,
): Map<string, number> | null {
  if (qbits === 0) return null;
  const effectiveCbits = cbits === 0 ? qbits : cbits;
  const circuit = new Circuit(qbits, effectiveCbits);
  for (const item of parsed) {
    if ("operation" in item) {
      circuit.push(item.operation);
    }
  }
  if (cbits === 0) {
    for (let i = 0; i < qbits; i++) {
      circuit.meas(i, i);
    }
  }
  const results = circuit.sample(1000);
  circuit.free();
  return results;
}

function deriveState(code: string) {
  const parsed = parse(code);
  const { qbits, cbits } = circuitSize(parsed);
  const drawing = drawCircuit(parsed, qbits, cbits);
  const samples = sampleCircuit(parsed, qbits, cbits);
  return { parsed, qbits, cbits, drawing, samples };
}

export const useCodeStore = create<CodeState>((set) => {
  const initial = localStorage.getItem("code") ?? "h 0\ncx 0 1";
  return {
    code: initial,
    ...deriveState(initial),
    setCode: (code) => {
      localStorage.setItem("code", code);
      set({ code, ...deriveState(code) });
    },
  };
});
