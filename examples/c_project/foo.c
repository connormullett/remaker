
#include <stdio.h>

#include "math.h"

int main() {
    char* foo = "foo";
    printf("%s\n", foo);

    int x = 5;
    int y = 5;
    int z = add(x, y);
    printf("%d\n", z);
}
