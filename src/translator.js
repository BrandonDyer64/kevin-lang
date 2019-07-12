const crypto = require("crypto");
const fun = require("./translations/function");

function translate(astIn) {
  return {
    newAst: fun(astIn.globals[0], { varTypes: {} }),
    hash: hashAST(astIn)
  };
}

function hashAST(ast) {
  const hash = crypto.createHash("md5");
  hash.update(JSON.stringify(ast));
  return hash.digest("hex").substr(0, 16);
}

module.exports = {
  translate
};
