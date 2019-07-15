const crypto = require("crypto");

function int(ast, state) {
  return {
    type: ast.staticType,
    compiled: `${ast.v}`
  };
}

function String(ast, state) {
  return {
    type: ast.staticType,
    compiled: `String(u8"${ast.v}")`
  };
}

function symbol(ast, state) {
  const hash = crypto.createHash("md5");
  hash.update(ast.v);

  return {
    type: "symbol",
    compiled:
      "0x" +
      hash
        .digest("hex")
        .substr(0, 8)
        .toUpperCase()
  };
}

module.exports = {
  int,
  String,
  symbol
};
