
const onCompareCheckboxChange = event => {
    const compareContainer = document.getElementById('compare-container');

    if (event.target.checked) {
        compareContainer.style.display = 'block';
    } else {
        compareContainer.style.display = 'none';
    }

    compare();
};

const compare = () => {
    const compareElement = document.getElementById('compare-value');

    const compareValue = compareElement.value;
    const originalValue = getOutputData();

    if (compareValue !== originalValue) {
        compareElement.classList.remove('compare-input-success');
        compareElement.classList.add('compare-input-fail');
    } else {
        compareElement.classList.remove('compare-input-fail');
        compareElement.classList.add('compare-input-success');
    }
};
