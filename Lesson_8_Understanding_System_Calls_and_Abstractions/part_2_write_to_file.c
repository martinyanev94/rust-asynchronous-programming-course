#include <stdio.h>

int main() {
    FILE *file = fopen("output.txt", "w");
    if (file == NULL) {
        perror("Error opening file");
        return 1;
    }

    fprintf(file, "Hello, world!\n");
    fclose(file);
    return 0;
}
