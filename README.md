# dprint-plugin-classname-wrap

A JSX classname wrapping plugin for [dprint](https://dprint.dev/).

---

## Installation

Add the plugin via dprint:

```sh
dprint config add liuhq/dprint-plugin-classname-wrap
```

Or manually add the plugin URL to your dprint configuration's `plugins` array. Replace `x.x.x` with the desired version:

```
https://plugins.dprint.dev/liuhq/classname-wrap-x.x.x.wasm
```

### Usage with `typescript` plugin

If you are using the `typescript` plugin, ensure `classname-wrap` is listed **after** `typescript`. This ensures classname formatting runs after other `typescript` code formatting.

Also, add the files you want to format to the `associations` of both `typescript` and `classnameWrap`.

```json
{
  "typescript": {
    "associations": ["**/*.tsx"]
  },
  "classnameWrap": {
    "associations": ["**/*.tsx"]
  },
  "plugins": [
    "https://plugins.dprint.dev/typescript-x.x.x.wasm",
    "https://plugins.dprint.dev/liuhq/classname-wrap-x.x.x.wasm"
  ]
}
```

## Configuration

|Option|Type|Description|Default|
|---|---|---|---|
|`classnameAttributes`|`string[]`|List of JSX attributes to format|`["class", "className"]`|
|`enableWrap`|`boolean`|Enable line wrapping|`true`|
|`allowLineOverflow`|`boolean`|Allow the last class to exceed `lineWidth`|`false`|
|[`indentToQuote`](#indentToQuote)|`boolean`|Indent lines aligned to the quote (`true`) or normally (`false`)|`true`|
|`indentWidth`|`number`|Number of spaces per indent|`2`|
|`lineWidthIncludesIndent`|`boolean`|Include indentation in `lineWidth` calculation|`false`|
|`lineWidth`|`number`|Maximum line width|`120`|

### Example

#### indentToQuote

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

## Build

Using [just](https://github.com/casey/just):

```sh
just build-release
```

Or with `cargo`:

```sh
cargo build --target wasm32-unknown-unknown --features "wasm" --release
```

## License

[MIT](./LICENSE)
