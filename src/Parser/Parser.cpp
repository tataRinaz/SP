#include "Parser.hpp"
#include <cassert>
#include <stdexcept>
#include <algorithm>


namespace SP {

BinaryOperation::BinaryOperation(Operation op, NodePtr leftPtr, NodePtr rightPtr) :
  operation(op), left(leftPtr), right(rightPtr)
{
}

std::string BinaryOperation::toString() const
{
  assert(left);
  assert(right);
  char oper;
  switch (operation)
  {
    case Operation::Plus:
      oper = '+'; break;
    case Operation::Minus:
      oper = '-'; break;
    case Operation::Divide:
      oper = '/'; break;
    case Operation::Multiply:
      oper = '*'; break;
    case Operation::Greater:
      oper = '>'; break;
    case Operation::Less:
      oper = '<'; break;
    default:
      assert(false);
  }

  return left->toString() + oper + right->toString();
}

std::optional<float> BinaryOperation::evaluate(Context& context) const
{
  assert(left);
  assert(right);

  auto leftEvaluated = left->evaluate(context);
  auto rightEvaluated = right->evaluate(context);
  
  if (!leftEvaluated.has_value() || !rightEvaluated.has_value())
  {
    throw std::runtime_error("Invalid binary operation");
  }

  auto& leftVal = leftEvaluated.value();
  auto& rightVal = rightEvaluated.value();

  switch (operation)
  {
  case Operation::Plus:
    return leftVal + rightVal;
  case Operation::Minus:
    return leftVal - rightVal;
  case Operation::Divide:
    return leftVal / rightVal;
  case Operation::Multiply:
    return leftVal * rightVal;
  case Operation::Greater:
    return leftVal > rightVal;
  case Operation::Less:
    return leftVal < rightVal;
  default:
    assert(false);
    throw std::runtime_error("Unexpected binary operation");
  }
}

std::string Number::toString() const
{
  if (fabs(value - static_cast<int>(value)) < 0.0000001)
    return std::to_string(static_cast<int>(value));
  return std::to_string(value);
}

std::optional<float> Number::evaluate(Context& context) const
{
  return value;
}

static BinaryOperation::Operation operationFromChar(char tokenType)
{
  switch (tokenType)
  {
  case '+':
    return BinaryOperation::Plus;
  case '-':
    return BinaryOperation::Minus;
  case '/':
    return BinaryOperation::Divide;
  case '*':
    return BinaryOperation::Multiply;
  case '<':
    return BinaryOperation::Less;
  case '>':
    return BinaryOperation::Greater;
  default:
    throw std::runtime_error("Unknown operation");
  }
}

using VecIter = std::vector<Token>::const_iterator;
static NodePtr simpleParse(VecIter begin, VecIter end)
{
  using TokenType = Token::TokenType;
  
  std::shared_ptr<Node> leftOperand = nullptr;
  std::optional<BinaryOperation::Operation> oper;
  for (; begin != end; ++begin)
  {
    auto& token = *begin;
    switch (token.type)
    {
    case TokenType::operation:
    {
      oper = operationFromChar(std::get<char>(token.value));
    }break;

    case TokenType::number:
    {
      auto numPtr = std::make_shared<Number>();
      numPtr->value = std::get<double>(token.value);

      if (!oper.has_value())
      {
        if (leftOperand)
          throw std::runtime_error("Parsing error: expected binary operation not number");
        leftOperand = numPtr;
      }
      else
      {
        auto operation = std::make_shared<BinaryOperation>(oper.value(), leftOperand, numPtr);
        oper.reset();
        leftOperand = operation;
      }
    }break;

    default:
      throw std::runtime_error("Unexpected token");
    }
  }

  return leftOperand;
}

NodePtr Parse(const std::vector<Token>& tokens)
{
  auto lastFirstPriorityPos = tokens.begin();
  auto end = tokens.end();

  NodePtr currentState = nullptr;
  std::optional<BinaryOperation::Operation> currentOper;

  while (lastFirstPriorityPos != end) {
    auto firstPriorityPos = std::find_if(lastFirstPriorityPos + 1, end,
      [](const Token token)
      {
        if (token.type != Token::TokenType::operation)
          return false;
        auto val = std::get<char>(token.value);
        return (val == '/') || (val == '*');
      });

    if (firstPriorityPos == end) {
      if (end - lastFirstPriorityPos <= 2)
      {
        return currentState;
      }
      if (!currentState)
      {
        return simpleParse(tokens.begin(), tokens.end());
      }
      auto rightOperand = simpleParse(lastFirstPriorityPos + 3, end);
      currentState = std::make_shared<BinaryOperation>(currentOper.value(), currentState, rightOperand);
      return currentState;
    }

    if (firstPriorityPos == tokens.begin() ||
      firstPriorityPos + 1 == end)
    {
      throw std::runtime_error("Parsing error: unexpected operation");
    }

    auto extractNumber = [](const auto iter)
    {
      if (iter->type != Token::TokenType::number)
      {
        throw std::runtime_error("Parsing error: expected number");
      }

      auto numPtr = std::make_shared<Number>();
      numPtr->value = std::get<double>(iter->value);

      return numPtr;
    };

    auto leftNum = extractNumber(firstPriorityPos - 1);
    auto rightNum = extractNumber(firstPriorityPos + 1);
    auto operation = operationFromChar(std::get<char>(firstPriorityPos->value));

    auto operPtr = std::make_shared<BinaryOperation>(operation, leftNum, rightNum);
    if (firstPriorityPos - 2 != tokens.begin())
    {
      auto leftOperationPos = firstPriorityPos - 2;
      if (leftOperationPos->type != Token::TokenType::operation)
      {
        throw std::runtime_error("Parsing error: expected operation");
      }

      auto parsePos = currentState ? lastFirstPriorityPos + 3 : tokens.begin();

      auto leftOperand = simpleParse(tokens.begin(), firstPriorityPos - 2);
      auto leftOperation = operationFromChar(std::get<char>(leftOperationPos->value));

      auto subTreeHead = std::make_shared<BinaryOperation>(leftOperation, leftOperand, operPtr);
      if (!currentState)
      {
        currentState = subTreeHead;
      }
      else
      {
        currentState = std::make_shared<BinaryOperation>(currentOper.value(), currentState, subTreeHead);
      }
    }
    else
    {
      currentState = operPtr;
    }

    if (firstPriorityPos + 2 != end)
    {
      if ((firstPriorityPos + 2)->type != Token::TokenType::operation)
      {
        throw std::runtime_error("Parsing error: expected operation");
      }
      currentOper = operationFromChar(std::get<char>((firstPriorityPos + 2)->value));
    }

    lastFirstPriorityPos = firstPriorityPos;
  }

  return currentState;
}
}

 //  1+2*3-4+5*6
 //   
 //     
 //      - 
 //   /     \   
 //  +       +  
 // / \     / \ 
 //1   *   4   *
 //   / \     / \
 //  2   3   5   6