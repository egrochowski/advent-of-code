let {input} = require('./input/10');

function parseInput(input) {
    return input.trim().split("\n").map(line => line.split(" "));
}

function drawPixel(cycle) {
    cycle %= 40
    return cycle >= x-1 && cycle <= x + 1 ? "#" : ".";
}

function isCheckPoint(cycle, check) { return cycle === check && check <= 220 }

function isAddX(cmd) { return cmd === "addx"; }

function getSignalStrength(cycle, x) { return cycle * x; }

let v = 0; let x = 1; let i = 0;
let cycle = 1;
let check = 20;
let sum = 0;
let cooldown=0;
let cmds = {
    noop: 1,
    addx: 2
} 

input = parseInput(input);
let cmd = input[0][0];
let str = ""

while (i < input.length) {

    // If the current process is finished, 
    // carry out operations and start new process
    if (!cooldown) {

        if (isAddX(cmd)) { x += v; }

        cmd = input[i][0];
        cooldown = cmds[cmd];

        if (isAddX(cmd)) { v = Number(input[i][1]) }
        i++;
    } 

    // Get signal strength if we have reached
    if (isCheckPoint(cycle, check)) {
        sum += getSignalStrength(cycle, x);
        check += 40;
    } 
    
    // Draw pixel each cycle
    str += drawPixel(cycle-1);

    // Print row every 40th cycle
    if (cycle % 40 === 0) {
        console.log(str); 
        str = "";
    }
    
    cooldown--;
    cycle++;
}

console.log({sum})