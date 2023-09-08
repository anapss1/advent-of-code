const fs = require('fs');

function readInputTxt() {
    try {
        const data = fs.readFileSync('./input.txt', 'utf8');
        return data;
    } catch (err) {
        console.error(err);
    }
}

function main() {
    console.log(readInputTxt());
}

if (require.main === module) {
    main();
}
