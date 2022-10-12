import * as wasm from "algorithms-wasm";

const CURRENT_ALGORITHM_INFO = 'current-algorithm-info';
const CURRENT_ALGORITHM_IN = 'current-algorithm-in';
const CURRENT_ALGORITHM_OUT = 'current-algorithm-out';

console.log(wasm.simple_hash('crypto-helper'));

const go = document.getElementById('go');
go.addEventListener('click', () => {
  const sha1In = document.getElementById('sha1-indata').value;

  const hash = wasm.sha1(sha1In);
  const sha1Out = document.getElementById('sha1-outdata');
  sha1Out.innerText = hash;
});
