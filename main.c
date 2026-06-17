#include <stdio.h>
#include <lexer.h>

char* TEST_CODE = "+";

int main(){
    Lexer lexer = {0, 1, TEST_CODE};
    loop(lexer);
    return 0;
}