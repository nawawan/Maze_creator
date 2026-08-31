# Add markdown parser wasm crate

## Overview

Added a new WebAssembly crate at `frontend/src/blog/wasm/` that parses markdown and returns HTML strings.

## Files

### `frontend/src/blog/wasm/Cargo.toml`

New crate definition with the following dependencies:

- `pulldown-cmark` — markdown parser (with `html` feature enabled)
- `wasm-bindgen` — exposes Rust functions to JavaScript
- `console_error_panic_hook` — surfaces Rust panics in the browser console
- `wasm-logger` / `log` — logging support

### `frontend/src/blog/wasm/src/lib.rs`

Exposes two `#[wasm_bindgen]` functions:

| Function | Signature | Description |
|---|---|---|
| `start` | `() -> ()` | Initialises panic hook and logger on wasm load |
| `parse_markdown` | `(&str) -> String` | Converts a markdown string to an HTML string |

`Options::all()` is passed to the parser, enabling all `pulldown-cmark` extensions: tables, footnotes, strikethrough, task lists, smart punctuation, and heading attributes.

## Usage

Build with `wasm-pack`:

```bash
cd frontend/src/blog/wasm
wasm-pack build --target web
```

Then import in JavaScript/TypeScript:

```ts
import init, { parse_markdown } from "./wasm/pkg/markdown_parser";

await init();
const html = parse_markdown("# Hello\n\nThis is **bold**.");
console.log(html);
// <h1>Hello</h1>
// <p>This is <strong>bold</strong>.</p>
```
