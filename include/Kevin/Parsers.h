#ifndef Parsers_h
#define Parsers_h

#include "llvm/ADT/APFloat.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/IR/BasicBlock.h"
#include "llvm/IR/Constants.h"
#include "llvm/IR/DerivedTypes.h"
#include "llvm/IR/Function.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/Type.h"
#include "llvm/IR/Verifier.h"

#include "./State.h"
#include "./Error.h"

#define Filter(x) if(!(x)) return false
#define Resolve(x) state = new_state; result = x; return true

bool Eat(string, string, State&, string&);
bool Eat(string, string, State&);
bool EatWhite(string, State&);

bool EatFunction(string, State&, string&);
bool EatFunctionName(string, State&, string&);

bool EatScope(string, State&, string&);
bool EatBracketScope(string, State&, string&);
bool EatExpressionScope(string, State&, string&);

bool EatStatementAssignment(string, State&, string&);

string &Indent(string &);

#endif
