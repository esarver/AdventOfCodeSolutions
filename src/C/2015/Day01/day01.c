#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../../COMMON/loadinput.c"

const char * year = "2015";
const char * day = "1";

const char upchar = '(';
const char dnchar = ')';
const int upval = 1;
const int dnval = -1;

void setup(void){
    set_input_root("../../../../inputs/");
}

int end_floor(int startFloor, char * input){
    int result = startFloor;
    
    int i = 0;
    while(input[i] != '\0'){
        switch(input[i]){
            case '(':
                result += upval;
                break;
            case ')':
                result += dnval;
                break;
            default:
                break;
        }
        ++i;
    }
    return result;
}

int first_index_at_floor(int startFloor, int desiredFloor, char * input){
    int index = 0;

    int currentFloor = startFloor;

    do{ 
        switch( input[index] ) {
            case '(':
                currentFloor += upval;
                break;
            case ')':
                currentFloor += dnval;
                break;
            default:
                break;
        }

        ++index;
    } while( currentFloor != desiredFloor );
    return index;
}

int main(void) {
    setup();

    char * input = loadInput(year, day);

    int endfloor = end_floor(0, input);

    printf("PART 1: final floor = %d\n", endfloor);

    int desiredFloor = -1;

    int firstTimeAt = first_index_at_floor(0, desiredFloor, input);

    printf("PART 2: first time at floor %d = %d\n", desiredFloor, firstTimeAt); 

    free(input);
    return 0;
}
