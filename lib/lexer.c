#include <tokens.h>
#include <lexer.h>
#include <utils/arraylist.h>
#include <stdio.h>
#include <string.h>

int cursor = 0;
int line = 1;

// the lexer loops over the code's lines and turns everything into tokens.
char currentChar(char* source){
    if(cursor >= strlen(source)){
        return '\0';
    }

    return source[cursor];
}

Token lexerize(char* source){
    switch (currentChar(source))
    {
    case '+':
        Token token = {TYPE_PLUS, "+", line};
        cursor += 1;
        return token;
    }
}

void loop(char* source){
    List tokenList;
    list_init(&tokenList, 2);

    while(currentChar(source) != '\0'){
        Token token = lexerize(source);
        list_add(&tokenList, token);

        printf("%s\n", list_get(&tokenList, 0).value);
    }

    freeList(&tokenList);
}