const literals = require("./literals");
const indent = require("../util/indent");

const primitives = [

];

function lambda(ast, state, utils) {
  console.log(utils);
  let returnType = ast.returnType;
  ast.scope.forEach(element => {
    if (element.type !== "return") return;
    const type = expression(element.expr, state, utils).type;
    console.log(type);
    if (!returnType) returnType = type;
    if (type != returnType) throw "Type mismatch";
  });
  const returnClause = returnType ? ` -> ${returnType}` : '';
  const paramList = ast.params.map(p => `${p.type} ${p.name}`).join(", ");
  const typeList = ast.params.map(p => p.type).join(",");
  return {
    type: `std::function<${returnType || 'void'}(${typeList})>`,
    compiled: `[](${paramList})${returnClause} {\n${indent(utils.scope(ast.scope, { ...state, returnType: null }, utils).compiled)}\n}`
  };
}

function literal(ast, state, utils) {
  return literals[ast.staticType](ast, state, utils);
}

const operatorMap = {
  "+": "Add",
  "-": "Sub",
  "*": "Mult",
  "/": "Div",
  "%": "Mod"
};

function operator(ast, state, utils) {
  const left = expression(ast.left, state, utils);
  const right = expression(ast.right, state, utils);
  if (left.type == right.type && ['int'].includes(left.type)) {
    return {
      type: left.type,
      compiled: `(${left.compiled} ${ast.type} ${right.compiled})`
    };
  } else {
    return {
      type: left.type,
      compiled: `${operatorMap[ast.type]}(${left.compiled}, ${right.compiled})`
    };
  }
}

function variable(ast, state, utils) {
  if (!state.varTypes[ast.name]) {
    throw `Variable '${ast.name}' hasn't been defined`;
  }
  return {
    type: state.varTypes[ast.name].type,
    compiled: ast.name
  };
}

function function_call(ast, state, utils) {
  const args = ast.args.map(arg => expression(arg, state, utils).compiled);
  return {
    type: "int",
    compiled: `${ast.name}(${args.join(", ")})`
  };
}

const types = {
  lambda,
  literal,
  variable,
  function_call
};

function expression(ast, state, utils) {
  if (ast.type in types) return types[ast.type](ast, state, utils);
  else if (ast.type in operatorMap) return operator(ast, state, utils);
  else throw `Unknown expression '${ast.type}'`;
}

module.exports = expression;
