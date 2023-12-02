let { input } = require("./input/09");

function parseInput(input) { return input.trim().split("\n"); }

function generateKnots(n) { return Array.from(Array(n), () => [0,0]); }

function serialize(tail) { return tail.join('.'); } 

function getRowDifference(x1, x2) { return Math.abs(x1 - x2); }

function getColDifference(y1, y2) { return Math.abs(y1 - y2); }

function isSameRow(headRow, tailRow) { return !getRowDifference(headRow, tailRow); }

function isSameCol(headCol, tailCol) { return !getColDifference(headCol, tailCol); }

function isLessThanTwoSteps(head, tail) { return getDistance(head, tail) < 2; }

function getDistance([x1, y1], [x2, y2]) {
    return Math.max(getRowDifference(x1, x2), getColDifference(y1, y2));
}

function moveTail([headRow, headCol], [tailRow, tailCol]) {

    // If the head adjacent to the tail, don't move it the tail's location  
    if (isLessThanTwoSteps([headRow, headCol], [tailRow, tailCol])) {
        return [tailRow, tailCol]
    }

    // If the head col is greater than the tail 
    // (i.e. the head is to the right of  the tail)
    // move the tail one space to the right
    // else move the tail to the left
    if (isSameRow(headRow, tailRow)) {
        return [tailRow, headCol > tailCol ? tailCol + 1 : tailCol - 1];
    } 
    
    // if the head row is less than the tail row, 
    // (i.e. the head is above the tail)
    // move the tail up, else move the tail down
    if (isSameCol(headCol, tailCol)) {
        return [headRow < tailRow ? tailRow - 1 : tailRow + 1, tailCol]
    }

    // if the head is above the tail, the tail must move up one space
    // so we decrement the tail's row to move it up one. 
    // if the head column is less than the tail col, we must move the 
    // the tail diagonally to the left, but if it is greater, than 
    // we need to move the tail diagonally to the right
    if (headRow < tailRow) {
        return [tailRow - 1, headCol < tailCol ? tailCol - 1 : tailCol + 1];
    } 

    // if the head is below the tail, we must increment the tail's row
    // to move it down one space. 
    // If the head column is less than the tail column, than we must 
    // decrement the tail's column by one to move the tail diagonally to
    // the left. Otherwise the tail will be move to the bottom right
    if (headRow > tailRow) {
        return [tailRow + 1, headCol < tailCol ? tailCol - 1 : tailCol + 1]
    }
}

function runRopeSimulation(input, n) {
    input = parseInput(input)
    const commands = {
        "U": [-1, 0],
        "R": [0, 1],
        "D": [1, 0],
        "L": [0, -1]
    };
    let knots = generateKnots(n);
    let seen = new Set([serialize(knots[knots.length-1])]);
    for (let cmd of input) {
        let [move, num] = cmd.trim().split(" ");
        num = Number(num);
        for (let i = 0; i < num; i++) {
            const [row, col] = commands[move];
            knots[0][0] += row;
            knots[0][1] += col;
            for(let i = 0; i < knots.length-1; i++) {
                knots[i+1] = moveTail(knots[i], knots[i+1])
            }
            seen.add(serialize(knots[knots.length-1]));
        }
    }
    return seen.size;
}

console.log(runRopeSimulation(input, 2), runRopeSimulation(input, 10)); 