#include <tokens.h>
#include <stdio.h>
#include <stdlib.h>

#ifndef LIST_H
#define LIST_H

/// @brief The struct that defines the List (don't mind, it'll only be used by Token)
///
/// The List struct has the following fields:
///
/// - array: Token*
///
/// - used: size_t
///
/// - size: size_t
typedef struct {
    Token* array;
    size_t used;
    size_t size;
} List;

/// @brief Initializes the list
/// @param list The list struct to be initialized
/// @param initialSize The initial size for the list
/// @warning initialSize will be soon be gone and the hardcoded size will be 10
void list_init(List* list, size_t initialSize);

/// @brief Adds token structs into the list, while also dynamically reallocating memory when necessary
/// @param list The list struct
/// @param token The token to be added into the list
void list_add(List* list, Token token);

/// @brief Retrieved token structs based on the position
///
/// The position starts at 0
/// @param list The list struct
/// @param pos The position to be retrieved
/// @return The token struct
Token list_get(List* list, int pos);

/// @brief Frees the list's memory, this is vital to stop memory leaks when the lexer is finished
/// @param list The list struct to be freed
void freeList(List* list);
#endif