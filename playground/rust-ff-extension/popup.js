import init, { add } from './pkg/rust_ff_extension.js';

async function main() {
  await init();

  document.getElementById('add').addEventListener('click', () => {
    const num1 = document.getElementById('num1').value;
    const num2 = document.getElementById('num2').value;
    const output = add(num1, num2);
    document.getElementById('output').textContent = output;
  });
}

main();
