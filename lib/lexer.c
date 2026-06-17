#include <tokens.h>
#include <lexer.h>
#include <utils/arraylist.h>
#include <stdio.h>
#include <string.h>

// the lexer loops over the code's lines and turns everything into tokens.
char currentChar(Lexer* lexer){
    if(lexer->cursor >= strlen(lexer->source)){
        return '\0';
    }

    return lexer->source[lexer->cursor];
}

Token lexerize(Lexer* lexer){
    switch (currentChar(lexer))
    {
    case '+':
        Token token = {TYPE_PLUS, "+", lexer->line};
        lexer->cursor += 1;
        return token;
    }
}

void loop(Lexer lexer){
    List tokenList;
    list_init(&tokenList, 2);

    while(currentChar(&lexer) != '\0'){
        Token token = lexerize(&lexer);
        list_add(&tokenList, token);

        printf("%s\n", list_get(&tokenList, 0).value);
    }

    freeList(&tokenList);
}