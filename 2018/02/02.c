#include <stdio.h>
#include <string.h>

size_t id_diff(char *first, char *second)
{
    int num_different = 0;

    while (*first && *second) {
        if (*(first++) != *(second++)) {
            num_different++;
        }
    }

    return num_different + strlen(first) + strlen(second);
}

void id_common(char *first, char *second)
{
    while (*first && *second) {
        if (*(first++) == *(second++)) {
            putchar(first[-1]);
        }
    }
}

int main()
{
    FILE *fp;
    char lines[250][27];

    fp = fopen("input.txt", "r");

    for (int i = 0; i < 250; i++) {
        fgets(lines[i], 27, fp);
    }

    fclose(fp);

    for (int i = 0; i < 250; i++) {
        for (int j = i + 1; j < 250; j++) {
            if (id_diff(lines[i], lines[j]) == 1) {
                id_common(lines[i], lines[j]);
            }
        }
    }
}

