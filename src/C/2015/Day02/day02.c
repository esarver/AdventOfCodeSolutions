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
    if (*ia > *ib) {
        return 1;
    } else if ( *ia < *ib) {
        return -1;
    } else {
        return 0;
    }
}

/**
* Calculate the unique box sides and sort them from smallest
* to largest.
*/
int * box_sides(Box box){
    int * unique_sides = malloc(3 * sizeof(int));
    unique_sides[0] = 0;
    unique_sides[1] = 0;
    unique_sides[2] = 0;

    if(box.l != 0 && box.w != 0 && box.h != 0){    
        unique_sides[0] = box.l * box.w;
        unique_sides[1] = box.l * box.h;
        unique_sides[2] = box.w * box.h;

        qsort(unique_sides, 3, sizeof(int), compare_ints);

   }
    return unique_sides;
}

long long box_surface_area_and_extra( int * sides ){
    long long accumulated_sides = 0;

    if(sides[0] != 0 && sides[1] != 0 && sides[2] != 0){
        accumulated_sides += 3 * (long long)sides[0]; // both smallest sides + 1 extra
        accumulated_sides += 2 * (long long)sides[1];
        accumulated_sides += 2 * (long long)sides[2];
    }

    return accumulated_sides;
}

/**
* Parse the string (e.g. "42x3x1") into 
* a Box struct.
*/
Box parse_box(char * box_string){
    // TODO - Parse the box_string
   char * box_cpy = strdup(box_string);
   
   const char delim[] = "x";

   Box b;

   char * dim_str;
   if( ( dim_str = strsep(&box_cpy, delim)) ){
        b.l = atoi( dim_str );
   }
   if( ( dim_str = strsep(&box_cpy, delim)) ){
        b.w = atoi( dim_str );
   }
   if( ( dim_str = strsep(&box_cpy, delim)) ){
        b.h = atoi( dim_str );
   }

   free(box_cpy);

   return b;
}

long long calculate_total_surface_area(char * input){
    long long accum = 0;
    const char delim[] = "\n";

    char * input_cpy = strdup(input);

    char * box_string;

    while( (box_string = strsep(&input_cpy, delim)) ) {
        accum += box_surface_area_and_extra( box_sides( parse_box( box_string))); 
    }

    free(input_cpy);

    return accum;
}

int main(void) {
    setup();

    long long result1 = calculate_total_surface_area(input);

    printf("PART 1: total surface area: [%lld]\n", result1);

    free(input);
    return 0;
}
