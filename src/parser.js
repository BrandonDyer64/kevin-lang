const fs = require('fs');
const peg = require('pegjs');

const globalPeg = `
_ = " "
`;

function validatePegFile(source, filename) {
  try {
    peg.generate(source + globalPeg);
    return true;
  } catch(e) {
    console.log(`In file ${filename}.peg`);
    console.log(e.message);
    console.log(source)
    return false;
  }
}

function getPegFile(filename, included = []) {
  return fs
    .readFileSync(`./src/grammar/${filename}.peg`, 'utf8')
    .replace(/\#include "(.*?)"/g, (x, incfile) => {
      if (included.includes(incfile)) return '';
      included.push(incfile);
      // if (!validatePegFile(getPegFile(incfile, [ filename ]), incfile)) {
      //   process.exit();
      // }
      return getPegFile(incfile, included);
    })
    + '\n';
}

module.exports = peg.generate(getPegFile('index'), { trace: true });;
