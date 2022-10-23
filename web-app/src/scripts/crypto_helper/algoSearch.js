
const onSearchCheckboxChange = () => {
    const searchBlock = document.getElementById('algo-search-block');

    if (document.getElementById('algo-search').checked) {
        searchBlock.style.display = 'flex';
    } else {
        searchBlock.style.display = 'none';
    }
}

const onAlgoSelect = algo => {
    console.log('on algo select');
    const algorithm = document.getElementById('algorithm');
    algorithm.value = algo;
    algorithm.selectedOptions[0].value = algo;
    algorithm.dispatchEvent(new Event('change'));
};

const SUPPORTED_ALGORITHMS = [
    'md5',
    'sha1',
    'sha256',
    'sha512',
    'aes128-cts-hmac-sha1-96',
    'aes256-cts-hmac-sha1-96',
]

const removeFoundChild = () => {
    const container = document.getElementById('algo-search-results');
    while (container.childNodes.length > 0) {
        container.removeChild(container.childNodes[0]);
    }
};

const onPatternChange = () => {
    const pattern = document.getElementById('algo-name-pattern').value;

    const container = document.getElementById('algo-search-results');
    removeFoundChild();

    if (pattern.length === 0) {
        return;
    }

    const matchedAlgos = SUPPORTED_ALGORITHMS.filter(algo => algo.startsWith(pattern)).map(algo => {
        const algoElement = document.createElement('span');
        algoElement.addEventListener('click', () => onAlgoSelect(algo));
        algoElement.innerText = algo.toUpperCase();
        return algoElement;
    });

    if (matchedAlgos.length > 0) {
        for (const child of matchedAlgos) {
            container.appendChild(child);
        }
    } else {
        const infoElement = document.createElement('span');
        infoElement.className = 'no-algo';
        infoElement.innerText = '-- No algorithm found --';
        container.appendChild(infoElement);
    }

};

const onPatternSubmit = (event) => {
    if (event.key === 'Enter') {
        const container = document.getElementById('algo-search-results');
        if (container.childNodes.length > 0) {
            container.childNodes[0].dispatchEvent(new Event('click'));
        }
    }
}
