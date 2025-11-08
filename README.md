# (WIP) dprint-plugin-tailwindcss

A Tailwind CSS classname formatting plugin for dprint, supporting class sorting and wrapping.

**Work in Progress**

- **Matcher**
  - [x] JSX Attributes: format classes in custom attributes
  - [ ] Functions: format classes in function calls, e.g., [clsx](https://github.com/lukeed/clsx)
- **Sorter**
  - [ ] Sort classes according to Tailwind CSS recommended class order
- **Wrapper**
  - [x] Automatically wrap lines when classes are too long

## Build

Using [just](https://github.com/casey/just):

```sh
just build-release
```

Or with `cargo` if `just` is not installed:

```sh
cargo build --target wasm32-unknown-unknown --features "wasm" --release
```

## Usage

> TODO

## Configuration

### Matcher

- `tailwindAttributes`: `string[]` - list of JSX attributes to format
- `tailwindFunctions`: `string[]` - list of functions to format

### Sorter

- `enableSort`: `boolean` - enable class sorting
- `sortVersion`: `"alphanumeric" | "v4"` - sorting strategy

### Wrapper

- `enableWrap`: `boolean` - enable line wrapping
- `allowLineOverflow`: `boolean` - allow the last class to exceed `lineWidth`
- `indentToQuote`: `boolean` - indent lines to class value quote
  - `true`:

  ```
    <div class="w-full ...
               h-full ...
  ```

  - `false`:

  ```
    <div class="w-full ...
      h-full ...
  ```

- `indentWidth`: `number` - number of spaces per indent
- `lineWidthIncludesIndent`: `boolean` - include indentation in `lineWidth` calculation
- `lineWidth`: `number` - maximum line width

## License

[MIT](./LICENSE)
