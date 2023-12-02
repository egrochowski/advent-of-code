let { input } = require('./input/06');

function findMarker(input, numDistinct) {
    let i = 0; let k = 1;
    let set = new Set(input[i]);
    while (i < input.length) {
        if (k - i === numDistinct) return k;
        if (set.has(input[k])) {
            set = new Set(input[++i]);
            k = i+1;
        } else {
            set.add(input[k++]); // Add unique char to set
        }
    }
    return -1;
}

console.log(findMarker(input,4),  findMarker(input, 14))
