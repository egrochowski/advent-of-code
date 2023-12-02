
// starting items -> worry levels
let { input } = require("./input/11")

let temp = input;
const COLON = ":";
const SPACE = " ";
const COMMA = ",";
const EQUAL = "=";

function setID(monkey) { return Number(monkey[0].split(SPACE)[1][0]); }

function isOld(x) { return isNaN(x); }

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

function inspectItems(monkey, product) {
    const { items } = monkey;
    const { op } = monkey;
    const divisor = product ? 1 : 3;
    let count = 0;
    while(items.length !== 0) {
        const item = items.pop();
        let newWorryLevel = Math.floor(operate(item, op[1], isOld(op[2]) ? item : op[2]) / divisor);
        if (product) newWorryLevel %= product;
        const monkeyTo = evalTest(monkey, newWorryLevel);
        throwTo(newWorryLevel, monkey.resultDest[monkeyTo]);
        count++;
    }
    return count;
}

function evalTest(monkey, newWorryLevel) {
    return Number(newWorryLevel % monkey.test === 0);
}

function getProductOfTests(monkeys) {
    return monkeys.reduce((product, monkey) => product * monkey.test, 1);
}

function simulateRounds(numRounds, pt2) {
    let k = 0; let n = input.length; let product;
    let inspections = Array.from(Array(n), () => 0);
    
    if (pt2) {
        product = getProductOfTests(input);
    };
    
    for (let i = 0; i < numRounds; i++) {
        for (let monkey of input) {
            inspections[(k++%n)] += inspectItems(monkey, product);
        }
    }
    return inspections;
}

// Part 1
input = parseInput(input);
const part1Result = simulateRounds(20).sort((a, b) => b - a);
console.log(part1Result[0] * part1Result[1])

// part 2
input = temp;
input = parseInput(input);
const part2Result = simulateRounds(10000, 1).sort((a, b) => b - a);
console.log(part2Result[0] * part2Result[1])