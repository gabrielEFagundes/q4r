#include <utils/arraylist.h>
#include <tokens.h>
#include <stdio.h>
#include <stdlib.h>

void list_init(List* list, size_t initialSize){
    list->array = (Token*) malloc(initialSize * sizeof(Token));
    list->used = 0;
    list->size = initialSize;
}

void list_add(List* list, Token token){
    if(list->used == list->size){
        int newSize = list->size * 2;
        Token* temp = (Token*) realloc(list->array, newSize * sizeof(Token));

        if(temp == NULL){
            fprintf(stderr, "couldn't allocate memory\n");
            exit(1);
        }

        list->array = temp;
        list->size = newSize;
    }

    list->array[list->used++] = token;
}

Token list_get(List* list, int pos){
    if(pos < 0 || pos >= list->used){
        fprintf(stderr, "array out of bounds\n");
        exit(1);
    }
    return list->array[pos];
}

void freeList(List* list){
    free(list->array);
    list->array = NULL;
    list->size = 0;
    list->used = 0;
}