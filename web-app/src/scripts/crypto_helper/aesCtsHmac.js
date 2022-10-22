
const onEncDec = id => {
    const checkbox = document.getElementById(`${id}-operation`).checked;
    
    if (checkbox) {
        document.getElementById(`${id}-cipher-len`).style.display = 'none';
        document.getElementById(`${id}-hmac`).innerText = '';
        document.getElementById(`${id}-hmac-len`).style.display = 'none';
    } else {
        document.getElementById(`${id}-cipher-len`).style.display = 'inline';
        document.getElementById(`${id}-hmac`).innerText = '00';
        document.getElementById(`${id}-hmac-len`).style.display = 'inline';
    }
};

const onPassOpts = id => {
    const checkbox = document.getElementById(`${id}-gen-key`).checked;
    
    if (checkbox) {
        document.getElementById(`${id}-pass-opts`).style.display = 'flex';
    } else {
        document.getElementById(`${id}-pass-opts`).style.display = 'none';
    }
};
