let { input } = require("./input/08");

function transformToMatrix(grid) {
    return grid
            .trim()
            .split("\n")
            .map(row => row.trim().split('').map(tree => Number(tree)));
}

function isVisible(grid, row, col) {
    const treeHeight = grid[row][col];
    return  checkRight(grid, row, col, treeHeight)[0]  ||
            checkLeft(grid, row, col, treeHeight)[0]   ||
            checkTop(grid, row, col, treeHeight)[0]    ||
            checkBottom(grid, row, col, treeHeight)[0]
}

function checkTop(grid, row, col, treeHeight) {
    for (let r = row - 1; r >= 0; r--) {
        if (grid[r][col] >= treeHeight){ 
            return [0, row - r]
        }; 
    }
    return [1, row];
}

function checkBottom(grid, row, col, treeHeight) {
    for (let r = row + 1; r < grid.length; r++) {
        if (grid[r][col] >= treeHeight) {
            return [0, r - row];
        }; 
    }
    return [1, grid.length - row - 1 ];
}

function checkRight(grid, row, col, treeHeight) {
    for (let c = col + 1; c < grid.length; c++) {
        if (grid[row][c] >= treeHeight) {
            return [0, c - col];
        } 
    }
    return [1, grid.length - col - 1];
}

function checkLeft(grid, row, col, treeHeight) {
    for(let c = col - 1; c >= 0; c--) {
        if (grid[row][c] >= treeHeight) {
            return [0, col - c];
        }; 
    }
    return [1, col];
}

function findNumTreesVisible(grid) {
    let sum = 0;
    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            sum += isVisible(grid, i, j)
        }
    }
    return sum;
}

function findArea(grid, i, j) {
    const treeHeight = grid[i][j];
    return checkTop(grid, i, j, treeHeight)[1]    *
           checkBottom(grid, i, j, treeHeight)[1] *
           checkLeft(grid, i, j, treeHeight)[1]   *
           checkRight(grid, i, j, treeHeight)[1]; 
}

function findMaxScenicScore(grid) {
    let max = grid[0][0];
    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            max = Math.max(max, findArea(grid, i, j))
        }
    }
    return max
}

input = transformToMatrix(input)
console.log(findNumTreesVisible(input), findMaxScenicScore(input))