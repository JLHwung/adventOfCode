const fs = require("fs");
const assert = require("assert");
const claims = fs.readFileSync("./input.txt", { encoding: "utf-8" }).split("\n").filter(x => x);

function parseClaim(claim) {
    const { groups: { id, left, top, width, height }} = /^#(?<id>\d+) @ (?<left>\d+),(?<top>\d+): (?<width>\d+)x(?<height>\d+)$/u.exec(claim);
    return {
        id: id,
        left: +left,
        top: +top,
        width: +width,
        height: +height
    }
}

const EMPTY = 0;
const CLAIMED = 1;
const OVERLAPPED = 2;


function initFabric(width, height) {
    return Array.from({ length: height }, () => Array.from({ length: width}, () => EMPTY));
}

function draw(state) {
    switch (state) {
        case EMPTY:
            return CLAIMED;
        case CLAIMED:
            return OVERLAPPED;
        case OVERLAPPED:
            return OVERLAPPED;
        default:
            assert.fail(`unreachable state ${state}`);
    }
}
let times = 0;
function drawClaim(claim, fabric) {
    for (let i = 0; i < claim.width; i++) {
        for (let j = 0; j < claim.height; j++) {
            fabric[claim.top + j][claim.left + i] = draw(fabric[claim.top + j][claim.left + i])
        }
    }
}

function summarizeOverlapped(fabric) {
    let sum = 0;
    for (let i = 0; i < 1000; i++) {
        for (let j = 0; j < 1000; j++) {
            if (fabric[i][j] === OVERLAPPED) {
                sum += 1;
            }
        }
    }
    return sum;
}

function main(rawClaims) {
    const claims = rawClaims.map(parseClaim);
    const fabric = initFabric(1000, 1000);
    for (const claim of claims) {
        drawClaim(claim, fabric);
    }
    return summarizeOverlapped(fabric);
}

console.log(main(claims));
