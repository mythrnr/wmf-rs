/* tslint:disable */
/* eslint-disable */

/**
 * Converts WMF binary data to an SVG string.
 *
 * # Arguments
 *
 * - `buf` - Byte array of a WMF file
 *
 * # Returns
 *
 * - SVG string (UTF-8)
 * - On failure, returns a JsValue containing error details
 *
 * # Example
 *
 * ```js
 * import { convertWmf2Svg } from "wmf-wasm";
 *
 * // svg is a string containing SVG data
 * const svg = convertWmf2Svg(wmfBytes);
 * ```
 */
export function convertWmf2Svg(buf: Uint8Array): string;

/**
 * Sets the log level (only when the `tracing` feature is enabled).
 *
 * # Arguments
 *
 * - `level` - e.g. "info", "debug", etc.
 *
 * # Example
 *
 * ```js
 * import { setLogLevel } from "wmf-wasm";
 * setLogLevel("debug");
 * // Now debug logs will be shown in the browser console (if tracing feature is enabled)
 * ```
 */
export function setLogLevel(level: string): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly convertWmf2Svg: (a: number, b: number, c: number) => void;
  readonly setLogLevel: (a: number, b: number) => void;
  readonly __wbindgen_export: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export2: (a: number) => void;
  readonly __wbindgen_export3: (a: number, b: number) => number;
  readonly __wbindgen_export4: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
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
