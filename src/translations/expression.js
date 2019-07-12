
const literals = require('./literals');

function literal(ast, state) {
  return literals[ast.staticType](ast, state);
}

const operatorMap = {
  "+": "Add",
  "-": "Sub",
  "*": "Mult",
  "/": "Div",
  "%": "Mod"
};

function operator(ast, state) {
  const left = expression(ast.left, state);
  const right = expression(ast.right, state);
  return {
    type: left.type,
    compiled: `${operatorMap[ast.type]}(${left.compiled}, ${right.compiled})`
  };
}

function variable(ast, state) {
  if (!state.varTypes[ast.name]) {
    throw `Variable '${ast.name}' hasn't been defined`;
  }
  return {
    type: state.varTypes[ast.name].type,
    compiled: ast.name
  }
}

function function_call(ast, state) {

}

const types = {
  literal,
  variable,
  function_call
};

function expression(ast, state) {
  if (ast.type in types)
    return types[ast.type](ast, state);
  else if (ast.type in operatorMap)
    return operator(ast, state);
  else throw `Unknown expression '${ast.type}'`;
};

module.exports = expression;
