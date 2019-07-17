const crypto = require("crypto");
const fun = require("./translations/function");

function translate(astIn) {
  const fun_c = fun(astIn.globals[0], { varTypes: {} });
  return {
    newAst: fun_c,
    compiled: pad(fun_c.compiled),
    hash: hashAST(astIn)
  };
}

function pad(source) {
  return `
#include <cstdint>
#include <iostream>
#include <string>
using std::int32_t;
using std::cout;
using std::endl;
using std::string;

namespace Console {
  using std::cout;
  using std::endl;
  #define Print(x) cout << x << endl
}

${source}
`;
}

function hashAST(ast) {
  const hash = crypto.createHash("md5");
  hash.update(JSON.stringify(ast));
  return hash.digest("hex").substr(0, 16);
}

module.exports = {
  translate
};
