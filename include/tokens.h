/// @brief Token used by the ast
typedef struct {
    const char* type;
    const char* value;
    const int line;
} Token;

/// @brief Type of the token
typedef enum {
    WORKSPACE,
    USE,

    INT,
    FLOAT,
    CHAR,
    STR,

    PLUS,
    MINUS,
    TIMES,
    DIVIDE,

    L_PAREN,    // (
    R_PAREN,    // )
    L_KEY,      // {
    R_KEY,      // }
    L_BRACKET,  // [
    R_BRACKET,  // ]

    EQ,
    EQEQ,
    LESSEREQ,
    LESSER,
    GREATEREQ,
    GREATER,
    DIFF,
    NOT,

    IF,
    ELSE,
    FOR,
    WHILE,
    F,
    RETURN
} TType;

struct Keyword{
    const char* name;
    const TType tokenType;
};

struct Keyword keywords[] = {
    {"IF",      IF},
    {"ELSE",    ELSE},
    {"FOR",     FOR},
    {"WHILE",   WHILE},
    {"F",       F},
    {"RETURN",  RETURN}
};