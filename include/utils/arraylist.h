#include <tokens.h>
#include <stdio.h>
#include <stdlib.h>

#ifndef LIST_H
#define LIST_H

// don't mind, it'll only be used for tokens right now
typedef struct {
    Token* array;
    size_t used;
    size_t size;
} List;

void list_init(List* list, size_t initialSize);

void list_add(List* list, Token token);

Token list_get(List* list, int pos);

void freeList(List* list);
#endif