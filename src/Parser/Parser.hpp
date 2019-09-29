#pragma once

#include <memory>
#include <map>

#include "Tokenizer.hpp"


namespace SP
{
//Abstract Syntax Tree
struct Node;
using NodePtr = std::shared_ptr<Node>;
struct Context
{
  std::map<std::string, NodePtr> functions;
  std::map<std::string, NodePtr> variables;
};

struct Node
{
  virtual std::string          toString() const = 0;
  virtual std::optional<float> evaluate(Context& context) const = 0;
};

struct BinaryOperation : public Node
{
  enum Operation
  {
    Plus,
    Minus,
    Divide,
    Multiply,
    Greater,
    Less
  };

  BinaryOperation(Operation op, NodePtr leftPtr, NodePtr rightPtr);
  virtual std::string toString() const override;
  virtual std::optional<float> evaluate(Context& context) const override;
  Operation operation;
  NodePtr left;
  NodePtr right;
};

struct Number : public Node
{
  virtual std::string toString() const override;
  virtual std::optional<float> evaluate(Context& context) const override;
  double value;
};


NodePtr Parse(const std::vector<Token>& tokens);
}


//    -
//   / \
//  +   3
// / \
//2   2
//We have 2 types: operation and operand
