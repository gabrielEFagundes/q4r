#include <tokens.h>

/// @brief Defines the structure of the Lexer
///
/// The Lexer struct has the following fields:
///
/// - cursor: int
///
/// - line: int
///
/// - source: char*
typedef struct{
    int cursor;
    int line;
    char* source;
} Lexer;

/// @brief Returns the current character based on the lexer's cursor
/// @param lexer The lexer struct
/// @return The current character the cursor is pointing to
char currentChar(Lexer* lexer);

/// @brief Lexerizes the raw code into defined tokens
/// @param lexer The lexer struct
/// @return The raw code turned into a Token
Token lexerize(Lexer* lexer);

/// @brief The loop that iterates through each character and line of the raw code
/// @param lexer The lexer struct
void loop(Lexer lexer);