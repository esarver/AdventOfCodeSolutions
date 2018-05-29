#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static char * _input_root = "./";

static void set_input_root(char * input_root){
    _input_root = input_root;
}

char * loadInput(char * year, char * day){

    char * buffer = 0;
    long length;

    char * filename = "input.txt";

    char * path = malloc(strlen(_input_root) + strlen(year) + 1 + strlen(day) + 1 + strlen(filename) + 1);

    strcpy(path, _input_root);
    strcat(path, year);
    strcat(path, "/");
    strcat(path, day);
    strcat(path, "/");
    strcat(path, filename);

    FILE * f = fopen(path, "rb");

    if (f){

        fseek(f, 0, SEEK_END);
        length = ftell(f);
        fseek(f, 0, SEEK_SET);
        buffer = malloc(length + 1);

        if (buffer){
            fread(buffer, 1, length, f);
        }
        fclose(f);
        //strcat(buffer, '\0');
    }

    return buffer;
}

//Test
/*
int main(void){
    set_input_root("../../../inputs/");
    char * input = loadInput("2015", "1");

    printf("INPUT:\n%s\n", input);

    free(input);

    return 0;
}
*/
