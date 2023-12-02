let rucksacks = require('./input/03').input;

function cleanInput(input) {
	return input.trim().split('\n');
}

function initPriorityMap() {
	const mapping = {};
	for(let i = 0; i < 52; i++) {
		mapping[getChar(i)] = (i+1);
	}
	return mapping;
}

function getChar(num) {
	let a = 'a'; 
	if (num > 25 ) {
		a = 'A';
		num -= 26;
	}
	return String.fromCharCode(a.charCodeAt(0) + num);
}

const rucksackCompartments = cleanInput(rucksacks)
	.map(rucksack => {
		const n = rucksack.length;
		const mid = Math.floor(n/2);
		return [rucksack.slice(0, mid) , rucksack.slice(mid)];
	});

function intersection(set1, set2) {
	return [...set1].filter(char => set2.has(char));
}

function findCommonChars(rucksack) {
	return intersection(new Set(rucksack[0]), new Set(rucksack[1]));
}

const mapping = initPriorityMap();

// part 1 
const part1Res = rucksackCompartments
	.reduce((sum, rucksack) => {
		return sum + findCommonChars(rucksack)
			.reduce(
				(prioritySum, char) => prioritySum + mapping[char], 0
			);
	}, 0)

// part 2
rucksacks = cleanInput(rucksacks);
let chars = []
for (let i = 0; i < rucksacks.length; i += 3) {
	let elf1 = new Set(rucksacks[i]);
	let elf2 = new Set(rucksacks[i+1]);
	let elf3 = new Set(rucksacks[i+2]);
	chars = [...chars, ...(intersection(intersection(elf1, elf2), elf3))]; 
}

const part2Res = chars.reduce((sum, char) => sum + mapping[char], 0);

console.log({part1Res}, {part2Res});
