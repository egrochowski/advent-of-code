let { input } = require('./input/07');

function cleanInput(input) {
    return input.trim().split("\n");
}

function newNode(name, parent=null, size=0, children=[]) {
    return { name, parent, size, children };
}

function isChange(line) { return line[1] === "cd" || line[1] === ".."; }

function goBack(line) { return line[2] === ".."; }

function isDir(line) { return line[0] === "dir";  }

function isSigil(line) { return line[0] === "$"; }

function getName(line) { return line[1]; }

function getSize(line) { return isDir(line) ? 0 : Number(line[0]); }

function hasChildren(node) { return !!node.children.length; }

function incrementParentSizes(curNode, size) {
    let temp = curNode;
    while (temp) {
        temp.size += size;
        temp = temp.parent;
    }
}

function findNode(curNode, name) {
    for (let node of curNode.children) {
        if (node.name === name) return node;
    }
    return curNode;
}

function buildTree(input) {
    let root = newNode("/");
    let curNode = root;
    let i = 1; 
    while (i < input.length) {
        const line = input[i++].trim().split(" ");
        if (isChange(line)) {
            curNode = goBack(line) 
                ? curNode.parent 
                : findNode(curNode, line[2]);
        } else if (!isSigil(line)) {
            const node = newNode(getName(line), curNode, getSize(line));
            curNode.children.push(node);
            incrementParentSizes(curNode, node.size);
        }
    }
    return root;
}

function getTotalDirectorySizesLessThanMax(root, max=100000) {
    if (!root) return 0;
    let sum = 0;
    for (let i = 0; i < root.children.length; i++) {
        const child = root.children[i];
        if (hasChildren(child)) {
            if (child.size <= max) sum += child.size; 
            sum += getTotalDirectorySizesLessThanMax(child);
        }
    }
    return sum;
};

function findMinDirToDelete(root) {
    let min = Infinity;
    const totalSpaceAvail = 70000000;
    const spaceRequiredToUpdate = 30000000;
    let spaceRequired = spaceRequiredToUpdate - (totalSpaceAvail - root.size);
    function traverse(root) {
        if (!root) return;
        min = root.size >= spaceRequired ? Math.min(min, root.size) : min;
        for (let i = 0; i < root.children.length; i++) {
            const child = root.children[i];
            if (hasChildren(child)) {
                traverse(child);
            }
        }
    }
    traverse(root);
    return min;
}

let root = (buildTree(cleanInput(input)));
console.log(getTotalDirectorySizesLessThanMax(root));
console.log(findMinDirToDelete(root))