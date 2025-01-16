# Example 1

## Requirements

Download and activate `emsdk`. Instructions can be found in the [repo](https://github.com/emscripten-core/emsdk).

## Compile to WASM

```
emcc fibonacci.c -O3 -s STANDALONE_WASM -s EXPORTED_FUNCTIONS='["_fibonacci"]' --no-entry -o fibonacci.wasm
```

## Test the page

```
python -m http.server
```

and then go to <http://[::]:8000/> in your browser.