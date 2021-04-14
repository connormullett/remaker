
#include <stdio.h>
#include "math.h"

int main() {
    char* foo = "foo";
    printf("%s\n", foo);

    int x, y = 5;
    int z = add(x, y);
    printf("%d%d%d\n", x, y, z);
}
