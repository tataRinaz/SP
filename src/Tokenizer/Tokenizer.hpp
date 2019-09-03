#include <vector>
#include <string>
#include <variant>
#include <optional>


namespace SP
{
  class Tokenizer
  {
    
    public:
      struct Token
      {
        using TokenValue = std::variant<char,
          double,
          std::string>;
        enum class TokenType
        {
          unknown,
          leftBracket,
          rightBracket,
          comma,
          number,
          keyword,
          identifier,
          operation
        };
        

        std::string ToString() const;

        bool operator==(const Token& rhs) const;
        TokenType type;
        TokenValue value;
      };

      Tokenizer() = default;
      Tokenizer(const Tokenizer&) = default;
      Tokenizer(Tokenizer&&) = default;
      Tokenizer& operator=(const Tokenizer&) = default;
      Tokenizer& operator=(Tokenizer&&) = default;

      std::vector<Token> Tokenize(const std::string& loc);
  private:
    using StrIterator = const char*;
    static std::optional<Token> parseToken(StrIterator& current,
                         StrIterator& end);
    static void skipSpaces(StrIterator& current, StrIterator& end);
  };
}