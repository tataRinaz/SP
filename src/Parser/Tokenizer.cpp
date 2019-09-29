#include "Tokenizer.hpp"

#include <cctype>
#include <array>
#include <algorithm>
#include <string_view>
#include <charconv>


namespace SP
{
  static std::array<std::string, 3> KEYWORDS
  {
    "func",
    "if",
    "else"
  };

  std::string Token::ToString() const
  {
#define TYPETOSTRING(TYPE) return std::string(#TYPE);
    TYPETOSTRING(type);
#undef TYPETOSTRING
  }

  bool Token::operator==(const Token& rhs) const
  {
    return type == rhs.type && value == rhs.value;
  }

  void Tokenizer::skipSpaces(Tokenizer::StrIterator& current, Tokenizer::StrIterator& end)
  {
    current = std::find_if_not(current, end, isspace);
  }

  std::optional<Token> Tokenizer::parseToken(Tokenizer::StrIterator& current,
                                                        Tokenizer::StrIterator& end) const
  {
    skipSpaces(current, end);
    if (current == end)
      return std::nullopt;

    if (isalpha(*current))
    {
      auto lexemAcceptorFn = [current, end](char ch) {
        return current != end && (isalnum(ch) || ch == '_');
      };
      auto lexemEnd = std::find_if_not(current, end, lexemAcceptorFn);

      std::string lexem(current, lexemEnd);
      current = lexemEnd;
      if (std::find(KEYWORDS.begin(), KEYWORDS.end(), lexem) == KEYWORDS.end()) {
        Token token{ Token::TokenType::keyword, lexem };
        return token;
      }
      else {
        Token token{ Token::TokenType::identifier, lexem};
        return token;
      }
    }

    if (isdigit(*current))
    {
      double value;
      auto [valuePtr, err]  = std::from_chars(current, end, value);
      if (err == std::errc())
      {
        current = valuePtr;
        Token token{ Token::TokenType::number, value };
        return token;
      }
      else
        throw std::runtime_error("NUMBER PARSING EXCEPTION");
    }

    auto ch = *current;
    current++;

    Token token;
    token.value = ch;
    switch (ch)
    {
    case '(':
      token.type = Token::TokenType::leftBracket; break;
    case ')':
      token.type = Token::TokenType::rightBracket; break;
    case '+':
    case '-':
    case '/':
    case '*':
    case '<':
    case '>':
      token.type = Token::TokenType::operation; break;
    case ',':
      token.type = Token::TokenType::comma; break;
    default:
      token.type = Token::TokenType::unknown; break;
    }
    return token;
  }


  std::vector<Token> Tokenizer::Tokenize(const std::string& loc) const
  {
    auto curr = loc.data();
    auto end = loc.data()+loc.size();

    std::vector<Token> tokens;
    while (auto token = parseToken(curr, end)) {
      tokens.push_back(token.value());
    }

    return tokens;
  }
}
