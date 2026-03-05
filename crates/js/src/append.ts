import type { Operation } from "./Operation";
export type { Operation } from "./Operation";
import type { Complex } from "./Complex";
export type { Complex } from "./Complex";
import type { ParsedOperation } from "./ParsedOperation";
export type { ParsedOperation } from "./ParsedOperation";
export type { Span } from "./Span";

type BitChar = "0" | "1";
type BitString = "" | `${BitChar}${BitString}`;
