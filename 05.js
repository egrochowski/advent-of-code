const { input } = require("./input/05");

function parseCommand(cmd) {
    return cmd
        .trim()
        .split(' ')
        .reduce((digits, str) => {
            return !isNaN(str) ? digits.concat(Number(str)) : digits
        }, [])
}

function extractChar(crate) {
  return crate[1]; // crate: [C] => C
}

function executeCommands(cmds, crates) {
	cmds = cmds.trim().split('\n');
	for (let cmd of cmds) {
		const [num, from, to] = parseCommand(cmd);
		crates = move(crates, num, from-1, to-1);
	}
	return crates
}

function executeCommands2(cmds, crates) {
	cmds = cmds.trim().split('\n');
	for (let cmd of cmds) {
		const [num, from, to] = parseCommand(cmd);
		crates = move2(crates, num, from-1, to-1);
	}
	return crates
}

function transpose(matrix, cols) {
	let newMatrix = [];
	for (let i = 0; i < cols; i++) {
		newMatrix.push([]);
	}

	for (let i = 0; i < matrix.length; i++) {
		let row = matrix[i].split(/\s+(?=\[)/g);
		for (let j = 0; j < cols; j++) {
			let char = extractChar(row[j]).trim();
			if (char) newMatrix[j].unshift(char);
		}
	}
	return newMatrix;
}

function peek(stack) {
	return stack[stack.length-1];
}

function move(crates, num, from, to) {
	for (let i = 0; i < num; i++) {
		let item = crates[from].pop()
		crates[to].push(item)
	}
	return crates
}

function move2(crates, num, from, to) {
	let temp = []
	for (let i = 0; i < num; i++) {
		temp.push(crates[from].pop())
	}
	while(temp.length) {
		crates[to].push(temp.pop())
	}
	return crates;
}

function getTopCrates(crates) {
	return crates
		.reduce((results, stack) => results.concat(peek(stack)), [])
		.join('')
}

function getNumCols(crates) {
	const row = crates.slice(-1)[0].trim();
	return Number(row[row.length-1]);
}

function formatInput(input) {
	for (let i = 0; i < input.length; i++) {
		let row = input[i].split('');
		for (let j = 0; j < row.length; j += 4) {
			if (row[j] !== '[') {
				row[j] = '['
				row[j+1] = ' '
				row[j+2] = ']'
			}
			input[i] = row.join('');
		}
	}
	return input;
}

// part 1
let [crates, cmds] = input.split('\n\n');

crates = crates
	.split('\n');

let numCols = getNumCols(crates);

crates = formatInput(crates)
	.slice(0,-1)
	.slice(1)

crates = transpose(crates, numCols)
crates = executeCommands(cmds, crates)

let part1Result = getTopCrates(crates);

// part 2
let [crates2, cmds2] = input.split('\n\n')
crates2 = crates2
	.split('\n');

numCols = getNumCols(crates2);

crates2 = formatInput(crates2)
	.slice(0,-1)
	.slice(1);

crates2 = transpose(crates2, numCols)
crates2 = executeCommands2(cmds2, crates2)
let part2Result = getTopCrates(crates2)
console.log({part1Result, part2Result})
