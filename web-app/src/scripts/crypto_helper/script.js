const CURRENT_ALGORITHM_INFO = 'current-algorithm-info';
const CURRENT_ALGORITHM_IN = 'current-algorithm-in';
const CURRENT_ALGORITHM_OUT = 'current-algorithm-out';

const getAlgorithm = () => document.getElementById('algorithm').selectedOptions[0].value;

const onAlgorithmChange = () => setAlgorithm(getAlgorithm());

const setAlgorithm = algorithm => {
  // set info elements
  const currentAlgorithmInfo = document.getElementById(CURRENT_ALGORITHM_INFO);
  currentAlgorithmInfo.removeAttribute('id');

  const newAlgorithmInfo = document.getElementsByName(algorithm + '-info')[0];
  newAlgorithmInfo.setAttribute('id', CURRENT_ALGORITHM_INFO);

  // set in elements
  const currentAlgorithmIn = document.getElementById(CURRENT_ALGORITHM_IN);
  currentAlgorithmIn.removeAttribute('id');

  const newAlgorithmIn = document.getElementsByName(algorithm + '-in')[0];
  newAlgorithmIn.setAttribute('id', CURRENT_ALGORITHM_IN);

  // set out elements
  const currentAlgorithmOut = document.getElementById(CURRENT_ALGORITHM_OUT);
  currentAlgorithmOut.removeAttribute('id');

  const newAlgorithmOut = document.getElementsByName(algorithm + '-out')[0];
  newAlgorithmOut.setAttribute('id', CURRENT_ALGORITHM_OUT);

  document.getElementById('go').click();
};

const copyOutputData = () => {
  const algorithm = getAlgorithm();
  if (algorithm.startsWith('sha')) {
    const outputData = document.getElementById(`${algorithm}-outdata`).innerText;
    console.log(outputData);
    navigator.clipboard.writeText(outputData);
  }
};
