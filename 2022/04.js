let { input } = require('./input/04')

function cleanInput(input) {
	return input
		.trim()
		.split('\n')
}

function getRange(elf) {
	return elf
		.split('-') 
		.map(digit => Number(digit)); 
}

function isFullyContained(pair) {
	let [elf1, elf2] = pair.split(',');
	let [x1, y1] = getRange(elf1);
	let [x2, y2] = getRange(elf2);
	return (x2 <= x1 && y1 <= y2) || (x1 <= x2 && y2 <= y1);
}

function hasOverlap(pair) {
	let [elf1, elf2] = pair.split(','); 
	let [x1, y1] = getRange(elf1);
	let [x2, y2] = getRange(elf2);
	return x1 <= y2 && y1 >= x2;
}

input = cleanInput(input);

const part1Res = input
	.reduce((sum, pair) => sum + !!isFullyContained(pair), 0);

const part2Res = input
	.reduce((sum, pair) => sum + !!hasOverlap(pair), 0);

console.log({part1Res, part2Res});
