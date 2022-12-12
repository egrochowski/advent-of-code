
// starting items -> worry levels
let { input } = require("./input/11")

let test = `
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
`

const COLON = ":";
const SPACE = " ";
const COMMA = ",";
const EQUAL = "=";


function setID(monkey) { return Number(monkey[0].split(SPACE)[1][0]); }

function isOld(x) { return isNaN(x); }

function getItemIndex(monkey, item) { return input[monkey].items.indexOf(item); }

function getItems(monkey) {
    const list = monkey[1]
                    .slice(monkey[1].indexOf(COLON)+1)
                    .trim()
                    .split(COMMA);
    return list.map(element => Number(element));
}

function getOp(monkey) {
    const list = monkey[2]
                    .slice(monkey[2].indexOf(EQUAL)+1)
                    .trim()
                    .split(SPACE)
    const element = list[list.length-1];
    list[list.length-1] = Number(element) || element;
    return list;
}

function getTest(monkey) {
    const list = monkey[3].trim().split(SPACE);
    return Number(list[list.length-1]);
}

function setResults(monkey) {
    const t = monkey[4].trim().split(SPACE);
    const f = monkey[5].trim().split(SPACE);
    return [
        Number(f[f.length-1]),
        Number(t[t.length-1])
    ];
}

function parseInput(input) {
    return input
            .trim()
            .split("\n\n")
            .map(monkey => monkey.trim().split("\n"))
            .map((monkey) => {
                return {
                    id: setID(monkey),
                    items: getItems(monkey),
                    op: getOp(monkey),
                    test: getTest(monkey),
                    resultDest: setResults(monkey)
                };
            });
}

function throwTo(item, monkey) {
    input[monkey].items.push(item);
}

function operate(old, op, x) {
    const ops = {
        "*": old * x,
        "+": old + x
    };
    return ops[op];
}


function inspectItems(monkey) {
    const { items } = monkey;
    const { op } = monkey;
    let count = 0;
    while(items.length !== 0) {
        const item = items.pop();
        let newWorryLevel = Math.floor(operate(item, op[1], isOld(op[2]) ? item : op[2]) / 3);
        const monkeyTo = evalTest(monkey, newWorryLevel);
        throwTo(newWorryLevel, monkey.resultDest[monkeyTo]);
        count++;
    }
    return count;
}

function evalTest(monkey, newWorryLevel) {
    return Number(newWorryLevel % monkey.test === 0);
}

function simulateRounds(numRounds) {
    let k = 0; let n = input.length;
    let inspections = Array.from(Array(n), () => 0);
    for (let i = 0; i < numRounds; i++) {
        for (let monkey of input) {
            inspections[(k++%n)] += inspectItems(monkey);
        }
    }
    return inspections;
}

// Part 1
input = parseInput(input);
const result = simulateRounds(20).sort( (a, b) => b - a);
console.log(result[0] * result[1])

