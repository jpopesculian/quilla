import type { Operation } from "./Operation";
export type { Operation } from "./Operation";
import type { Complex } from "./Complex";
export type { Complex } from "./Complex";

type BitChar = "0" | "1";
type BitString = "" | `${BitChar}${BitString}`;
