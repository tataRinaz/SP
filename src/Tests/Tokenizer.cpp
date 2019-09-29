#define CATCH_CONFIG_MAIN

#include "catch2/catch.hpp"
#include "Parser/Tokenizer.hpp"

TEST_CASE("Basic string", "Tokenizer")
{
  using Token = SP::Token;
  std::string loc = "1+1";
  std::vector<Token> expectedResult = {
    Token{Token::TokenType::number, 1. },
    Token{Token::TokenType::operation, '+'},
    Token{Token::TokenType::number, 1.}
  };
  SP::Tokenizer t;
  REQUIRE(t.Tokenize(loc) == expectedResult);
}


