import type { Operation } from "./Operation";
export type { Operation } from "./Operation";

type BitChar = "0" | "1";
type BitString = "" | `${BitChar}${BitString}`;
