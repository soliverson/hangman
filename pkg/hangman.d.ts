/* tslint:disable */
/* eslint-disable */
export function run(): void;
export class Hangman {
  free(): void;
  constructor(word: string, max_attempts: number);
  guess(letter: string): boolean;
  display_word(): string;
  attempts_left(): number;
  is_won(): boolean;
  is_lost(): boolean;
  get_guesses(): string;
  get_word(): string;
  get_score(): string;
  reset_game(new_word: string): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_hangman_free: (a: number, b: number) => void;
  readonly hangman_new: (a: number, b: number, c: number) => number;
  readonly hangman_guess: (a: number, b: number, c: number) => number;
  readonly hangman_display_word: (a: number) => [number, number];
  readonly hangman_attempts_left: (a: number) => number;
  readonly hangman_is_won: (a: number) => number;
  readonly hangman_is_lost: (a: number) => number;
  readonly hangman_get_guesses: (a: number) => [number, number];
  readonly hangman_get_word: (a: number) => [number, number];
  readonly hangman_get_score: (a: number) => [number, number];
  readonly hangman_reset_game: (a: number, b: number, c: number) => void;
  readonly run: () => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_5: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb3945fa02d6bbe60: (a: number, b: number) => void;
  readonly closure20_externref_shim: (a: number, b: number, c: any) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
