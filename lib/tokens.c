#include <tokens.h>

const char* tokenValues[] = {
    #define X(type, value) value,
    TTYPES(X)
    #undef X
};

struct Keyword keywords[] = {
    {"IF",          TYPE_IF},
    {"ELSE",        TYPE_ELSE},
    {"FOR",         TYPE_FOR},
    {"WHILE",       TYPE_WHILE},
    {"F",           TYPE_F},
    {"RETURN",      TYPE_RETURN},
    {"WORKSPACE",   TYPE_WORKSPACE},
    {"USE",         TYPE_USE},
    {"INT",         TYPE_INT},
    {"FLOAT",       TYPE_FLOAT},
    {"CHAR",        TYPE_CHAR},
    {"STR",         TYPE_STR},
    {"INT64",       TYPE_INT64}
};