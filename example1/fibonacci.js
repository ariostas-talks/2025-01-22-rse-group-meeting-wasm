const wasmFile = 'fibonacci.wasm';

fetch(wasmFile)
  .then(response => {
    if (!response.ok) {
      throw new Error(`Failed to load ${wasmFile}: ${response.statusText}`);
    }
    return response.arrayBuffer();
  })
  .then(bytes => WebAssembly.instantiate(bytes))
  .then(({ instance }) => {
    const wasmExports = instance.exports;

    document.getElementById('computeButton').addEventListener('click', () => {
        const inputText = document.getElementById('input').value;
        const inputNum = parseInt(inputText, 10);
        const res = wasmExports.fibonacci(inputNum);
        document.getElementById('output').innerHTML = `F<sub>${inputNum}</sub> = ${res}`;
    });
  
  })
  .catch(error => {
    console.error('Error loading or running the WebAssembly module:', error);
  });
