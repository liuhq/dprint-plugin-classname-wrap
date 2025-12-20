# dprint-plugin-classname-wrap

A JSX classname wrapping plugin for dprint.

## Install

```sh
dprint config add liuhq/dprint-plugin-classname-wrap
```

## Build

Using [just](https://github.com/casey/just):

```sh
just build-release
```

Or with `cargo` if `just` is not installed:

```sh
cargo build --target wasm32-unknown-unknown --features "wasm" --release
```

## Configuration

### Matcher

- `classnameAttributes`: `string[]` - list of JSX attributes to format

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
