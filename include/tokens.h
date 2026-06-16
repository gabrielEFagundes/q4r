/// @brief Types of tokens
/// 
/// I've used the X-Macro mapping, it maps the types at compile time, saving up a lot of performance, and it's also very beautiful
#ifndef TOKENS_H
#define TOKENS_H

#define TTYPES(X) \
    X(WORKSPACE,    "WORKSPACE") \
    X(USE,          "USE") \
    X(IDENTIFIER,   "IDENTIFIER") \
    X(INT,          "INT") \
    X(FLOAT,        "FLOAT") \
    X(CHAR,         "CHAR") \
    X(STR,          "STR") \
    X(INT64,        "INT64") \
    X(PLUS,         "PLUS") \
    X(MINUS,        "MINUS") \
    X(TIMES,        "TIMES") \
    X(DIVIDE,       "DIVIDE") \
    X(L_PAREN,      "L_PAREN") \
    X(R_PAREN,      "R_PAREN") \
    X(L_KEY,        "L_KEY") \
    X(R_KEY,        "R_KEY") \
    X(L_BRACKET,    "L_BRACKET") \
    X(R_BRACKET,    "R_BRACKET") \
    X(EQ,           "EQ") \
    X(EQEQ,         "EQEQ") \
    X(LESSEREQ,     "LESSEREQ") \
    X(LESSER,       "LESSER") \
    X(GREATEREQ,    "GREATEREQ") \
    X(GREATER,      "GREATER") \
    X(DIFF,         "DIFF") \
    X(NOT,          "NOT") \
    X(IF,           "IF") \
    X(ELSE,         "ELSE") \
    X(FOR,          "FOR") \
    X(WHILE,        "WHILE") \
    X(F,            "F") \
    X(RETURN,       "RETURN") \
    X(EOF,          "EOF") \
    X(ILLEGAL,      "ILLEGAL")

/// @brief Enum with the types (procedure generation per item on TTYPES)
typedef enum {
    #define X(type, value) TYPE_##type,
    TTYPES(X)
    #undef X
} TType;

/// @brief Collect the value of the token, which is basically just a string containing the name (why, though?)
extern const char* tokenValues[];

/// @brief Token used by the ast
typedef struct {
    TType type;
    const char* value;
    int line;
} Token;

/// @brief Defines the keyword structure
struct Keyword{
    const char* name;
    const TType tokenType;
};

/// @brief Defines the language's specific keywords
extern struct Keyword keywords[];
#endif