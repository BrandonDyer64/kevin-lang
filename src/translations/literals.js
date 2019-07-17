const crypto = require("crypto");

function int(ast, state) {
  let type = "int32_t";
  let value = ast.v;
  const match = ast.v.match(/([0-9]+)([iudf])([0-9]+)/);
  if (match) {
    value = match[1];
    type = {
      i: "int",
      f: "float",
      u: "uint"
    }[match[2]] + match[3] + "_t";
  }
  return {
    type,
    compiled: value
  };
}

function string(ast, state) {
  return {
    type: "string",
    compiled: `u8"${ast.v}"`
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
  string,
  symbol
};
