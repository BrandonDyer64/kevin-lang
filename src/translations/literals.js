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

module.exports = {
  int,
  String
};
