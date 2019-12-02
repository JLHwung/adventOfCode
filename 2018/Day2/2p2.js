const fs = require("fs");
const assert = require("assert");
const boxes = fs.readFileSync("./input.txt", { encoding: "utf-8" }).split("\n").filter(x => x);

function isStringDistanceGreaterThan(x, y, threshold) {
    assert.equal(x.length, y.length);
    let distance = 0;
    for (let i = 0; i < x.length; i++) {
        if (x[i] !== y[i]) {
            distance += 1;
        }
        if (distance > threshold) {
            return true;
        }
    }
    return false;
}

function findCorrectBoxes(boxes) {
    let i, j;
    for (i = 0; i < boxes.length - 1; i++) {
        for (j = i + 1; j < boxes.length; j++) {
            if (!isStringDistanceGreaterThan(boxes[i], boxes[j], 1)) {
                return [i, j];
            }
        }
    }   
    return [i, j];
}

function main(boxes) {
    const [i, j] = findCorrectBoxes(boxes);
    if (i < boxes.length && j < boxes.length) {
        console.log(boxes[i], boxes[j]);
    }
    console.log(i, j);
}

main(boxes);