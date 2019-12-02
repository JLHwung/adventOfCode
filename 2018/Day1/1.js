const fs = require("fs");
const frequencies = fs.readFileSync("./input.txt", { encoding: "utf-8" }).split("\n");

let sum = 0;
const results = new Set([sum]);
for (;;) {
    for (const frequency of frequencies) {
        sum += +frequency;
        if (results.has(sum)) {
            console.log("First frequency hit twice is ", sum);
            return;
        } else {
            results.add(sum);
        }
    }
}

// console.log(...results.values())