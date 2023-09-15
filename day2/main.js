const fs = require('fs');

const CHOICE_SCORE_TABLE = {
    X: 1,
    Y: 2,
    Z: 3,
}

const RESULT_SCORE_TABLE = {
    win: 6,
    draw: 3,
    loss: 0,
}

const ROUND_RESULT_TABLE = {
    A: {
        X: 'draw',
        Y: 'win',
        Z: 'loss'
    },
    B: {
        X: 'loss',
        Y: 'draw',
        Z: 'win'
    },
    C: {
        X: 'win',
        Y: 'loss',
        Z: 'draw'

    }
}

function readInputTxt() {
    try {
        const data = fs.readFileSync('./input.txt', 'utf8');
        return data;
    } catch (err) {
        console.error(err);
    }
}

function rpsRound(roundString) {
    let [other, me] = roundString.split(' ');
    let round_score = 0;
    let result = ROUND_RESULT_TABLE[other][me]
    round_score += RESULT_SCORE_TABLE[result]
    round_score += CHOICE_SCORE_TABLE[me];
    return round_score;
}

function main() {
    let inputLines = readInputTxt().trim().split(/\r?\n/);
    let score = 0;
    inputLines.forEach((line) => { score += rpsRound(line) })
    console.log(score);
}

if (require.main === module) {
    main();
}
