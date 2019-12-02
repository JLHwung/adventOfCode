const fs = require("fs");
const boxes = fs.readFileSync("./input.txt", { encoding: "utf-8" }).split("\n");

function findStringRepeatStats(string) {
    const repeats = new Map();
    for (const str of string) {
        if (!repeats.has(str)) {
            repeats.set(str, 0);
        }
        repeats.set(str, repeats.get(str) + 1);
    }
    const values = [...repeats.values()];
    return {
        two: values.includes(2),
        three: values.includes(3)
    };
}

function checkSum(boxes) {
    let twos = 0, threes = 0;
    for (const box of boxes) {
        const stats = findStringRepeatStats(box);
        if (stats.two) {
            twos++;
        }
        if (stats.three) {
            threes++;
        }
    }
    return twos * threes;
}

console.log(checkSum(boxes));