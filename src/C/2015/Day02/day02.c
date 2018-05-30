#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../../COMMON/loadinput.c"

char * year = "2015";
char * day = "2";

static char * input;

void setup(void){
    set_input_root("../../../../inputs/");
    input = loadInput(year, day);
}

typedef struct box{
    int l;
    int w;
    int h;
} Box;

/**
* Compares integers
*
*/
int compare_ints(const void *a, const void *b){
    const int *ia = (const int *) a;
    const int *ib = (const int *) b;

    return (*ia > *ib) - (*ia < *ib);
}

/**
* Calculate the unique box sides and sort them from smallest
* to largest.
*/
int * box_sides(Box box){
    int unique_sides[] = {0, 0, 0};

    unique_sides[0] = box.l * box.w;
    unique_sides[1] = box.l * box.h;
    unique_sides[2] = box.w * box.h;

    qsort(unique_sides, 3, sizeof(int), compare_ints);

    return unique_sides;
}

/**
* Parse the string (e.g. "42x3x1") into 
* a Box struct.
*/
Box parse_box(char * box_string){
    // TODO - Parse the box_string
}

char * extract_dimensions(char * input){
    // TODO - Do we want to separate out this functionality?
}	

long long calculate_total_surface_area(char * input){
    long long accum = 0;


    return accum;
}

int main(void) {
    setup();

    long long result1 = calculate_total_surface_area(input);

    printf("PART 1: total surface area: %lld", result1);

    free(input);
    return 0;
}
