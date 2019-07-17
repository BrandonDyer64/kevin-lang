const indent = require("../util/indent");
const scope = require("./scope");

module.exports = (ast, state) => {
  const params = ast.params.map(p => `${p.type} ${p.name}`).join(", ");

  const scopeAST = scope(ast.scope, {
    varTypes: ast.params.reduce((result, param) => {
      result[param.name] = {
        dec: "let",
        mut: null,
        type: param.type
      };
      return result;
    }, state.varTypes),
    returnType: ast.returnType,
    functionName: ast.name == 'Main' ? 'main' : ast.name,
    ...state
  }, {});

  return {
    compiled: `
${ast.returnType} ${ast.name == 'Main' ? 'main': ast.name}(${params})\n{
${indent(scopeAST.compiled)}
}
`
  };
};
