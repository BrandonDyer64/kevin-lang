const crypto = require('crypto');

function translate(astIn) {
  return {
    hash: hashAST(astIn)
  };
}

function hashAST(ast) {
  const hash = crypto.createHash('md5');
  hash.update(JSON.stringify(ast));
  return hash.digest('hex').substr(0, 16);
}

module.exports = {
  translate
};
