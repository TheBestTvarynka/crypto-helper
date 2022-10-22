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
  if (algorithm.startsWith('sha') || algorithm.startsWith('md5')) {
    const outputData = document.getElementById(`${algorithm}-outdata`).innerText;
    console.log(outputData);
    navigator.clipboard.writeText(outputData);

    showNotification({ type: 'info', text: 'Copied!' });
  } else if (algorithm.endsWith('-cts-hmac-sha1-96')) {
    let cipher = document.getElementById(`${algorithm}-cipher`).innerText;
    let hmac = document.getElementById(`${algorithm}-hmac`).innerText;
    navigator.clipboard.writeText(cipher + hmac);

    showNotification({ type: 'info', text: 'Copied!' });
  }
};

const KEY_USAGE_NAMES = {
  '1': 'AS-REQ PA-ENC-TIMESTAMP',
  '2': 'AS-REP Ticket',
  '3': 'AS-REP Enc part',
  '4': 'TGS-REQ KDC-REQ-BODY AuthData (session key)',
  '5': 'TGS-REQ KDC-REQ-BODY AuthData (authenticator subkey)',
  '6': 'TGS-REQ PA-TGS-REQ padata AP-REQ Authenticator cksum (session key)',
  '7': 'TGS-REQ PA-TGS-REQ padata AP-REQ Authenticator (session key)',
  '8': 'TGS-REP enc part (session key)',
  '9': 'TGS-REP enc part (authenticator subkey)',
  '10': 'AP-REQ Authenticator cksum (session key)',
  '11': 'AP-REQ Authenticator (session key)',
  '12': 'AP-REP enc part (session key)',
  '13': 'KRB-PRIV enc part',
  '14': 'KRB-CRED enc part',
  '15': 'KRB-SAFE cksum',
  '19': 'AD-KDC-ISSUED cksum',
  '22': 'KG-USAGE-ACCEPTOR-SEAL',
  '23': 'KG-USAGE-ACCEPTOR-SIGN',
  '24': 'KG-USAGE-INITIATOR-SEAL',
  '25': 'KG-USAGE-INITIATOR-SIGN',
  '41': 'PKU2U_KRB_FINISHED',
};

const onKeyUsageChange = algorithm => {
  let usage = document.getElementById(`${algorithm}-inusage`).value;
  document.getElementById(`${algorithm}-usage`).innerText = KEY_USAGE_NAMES[usage] || '?unknown?';
}

const onLenChange = (algorithm, type) => {
  const len = document.getElementById(`${algorithm}-${type}-input`).value.length / 2;
  console.log({ len });
  document.getElementById(`${algorithm}-${type}-len`).innerText = len;
};
