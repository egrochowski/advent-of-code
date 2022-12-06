let { input } = require('./input/01');

function getMaxCalories(list) {

	let max = 0;
	let sum = 0;
	let num = "";
	for(let i = 0; i < list.length; i++) {
		let char = list[i];
		if (char === "\n") {
			sum += Number(num);
			if (list[i+1] === "\n") {
				max = Math.max(sum, max);
				sum = 0;
				i++;
			}
			num = "";
		} else if (char !== " " && char !== "\n") {
			num += char
		}
	}
	sum += Number(num);
	max = Math.max(max, sum)
	return max;
}

function getMaxCalories2(list) {
	list = list.trim().split("\n\n");
	let max = 0;
	for(let group of list) {
		let sum = group.split("\n").reduce((acc, num) => acc + Number(num), 0);
		max = Math.max(max, sum);
	}
	return max;
}


function getTop3MaxCalories(list) {
	let maxNums = [0,0,0];
	let sum = 0;
	let num = "";

	for(let i = 0; i < list.length; i++) {
		let char = list[i];
		if (char === "\n") {
			sum += Number(num);
			if (list[i+1] === "\n") {
				maxNums.push(sum);
				maxNums.sort((a,b) => b-a).pop()
				sum = 0;
			}
			num = "";
		} else if (char !== " " && char !== "\n") {
			num += char
		}
	}

	maxNums.push(sum);
	maxNums.sort((a,b) => b-a).pop()

	return maxNums.reduce((a, b) => a + b, 0);
}

console.log(getMaxCalories2(input)); 
console.log(getTop3MaxCalories(input));
