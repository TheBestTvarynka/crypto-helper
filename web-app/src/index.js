import * as wasm from "algorithms-wasm";

const CURRENT_ALGORITHM_INFO = 'current-algorithm-info';
const CURRENT_ALGORITHM_IN = 'current-algorithm-in';
const CURRENT_ALGORITHM_OUT = 'current-algorithm-out';

try {
  console.log(wasm.simple_hash('crypto-helper'));
} catch(e) {
  console.dir({ e });
}

const getAlgorithmFn = algorithm => algorithm;

const getAlgorithm = () => document.getElementById('algorithm').selectedOptions[0].value;

const collectData = algorithm => {
  if (algorithm.startsWith('sha')) {
    return [document.getElementById(`${algorithm}-indata`).value];
  } else {
    throw new Error('not implemented yet');
  }
};

const setData = (algorithm, data) => {
  if (algorithm.startsWith('sha')) {
    document.getElementById(`${algorithm}-outdata`).innerText = data;
  } else {
    throw new Error('not implemented yet');
  }
};

const go = () => {
  console.log('go');
  const algorithm = getAlgorithm();
  const dataIn = collectData(algorithm);
  try {
    const dataOut = wasm[getAlgorithmFn(algorithm)](...dataIn);
    setData(algorithm, dataOut);
  } catch(e) {
    console.error(e);
  }
};

document
  .getElementById('go')
  .addEventListener('click', go);

document
  .getElementById('autoConvert')
  .addEventListener('change', () => {
    const algorithm = getAlgorithm();
    if (document.getElementById('autoConvert').checked) {
      if (algorithm.startsWith('sha')) {
        const indata = document.getElementById(`${algorithm}-indata`);
        indata.addEventListener('change', go);
        indata.addEventListener('input', go);
      }
    } else {
      if (algorithm.startsWith('sha')) {
        const indata = document.getElementById(`${algorithm}-indata`);
        indata.removeEventListener('change', go);
        indata.removeEventListener('change', go);
      }
    }
  })