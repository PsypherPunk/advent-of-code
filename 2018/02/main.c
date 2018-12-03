#include <stdio.h>
#include <err.h>

int char_count(char *line, char c)
{
    int count = 0;

    while (*line) {
        if (*(line++) == c) {
            count++;
        }
    }

    return count;
}


int main()
{
    FILE *fp;
    char line[27];
    char c;
    int twice, thrice = 0;
    int twice_count, thrice_count = 0;
    int count;

    fp = fopen("input.txt", "r");

    while (fgets(line, 27, fp)) {
        twice = 0;
        thrice = 0;
        for (c = 'a'; c <= 'z'; c++) {
            count = char_count(line, c);
            if (count == 3) {
                thrice = 1;
            } else if (count == 2) {
                twice = 1;
            }
        }
        if (twice) {
            twice_count++;
        }
        if (thrice) {
            thrice_count++;
        }
    }
    fclose(fp);

    printf("%d\n", twice_count * thrice_count);
}

