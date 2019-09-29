#include "catch2/catch.hpp"
#include "Parser.hpp"

namespace SP {

static void checkExpression(std::string_view expression, double expected_result)
{
  Context context;
  Tokenizer t;

  auto tokens = t.Tokenize(std::string(expression));

  auto ast = Parse(tokens);
  REQUIRE(ast->toString() == expression);
  REQUIRE(ast->evaluate(context) == expected_result);
}

TEST_CASE("2+2", "Parser")
{
  checkExpression("2+2", 4.);
}

TEST_CASE("2+2+2", "Parser")
{
  checkExpression("2+2+2", 6.);
}

TEST_CASE("1+2*3", "Parser")
{
  checkExpression("1+2*3", 7.);
}

TEST_CASE("1+2*3-4+5*6", "Parser")
{
  checkExpression("1+2*3-4+5*6", 33.);
}

TEST_CASE("1+2*3-4+5*6-7", "Parser")
{
  checkExpression("1+2*3-4+5*6-7", 26.);
}


TEST_CASE("Unexpected number(2+2 2)", "Parser")
{
  Context context;

  const std::vector<Token> tokens = {
    Token{Token::TokenType::number, 2. },
    Token{Token::TokenType::operation, '+'},
    Token{Token::TokenType::number, 2.},
    Token{Token::TokenType::number, 2.}
  };

  REQUIRE_THROWS(Parse(tokens));
}
}