#include <stdio.h>
#include <time.h>

#define MS_PER_SEC 1000

static int euclid(int a, int b)
{
    int temp;
    while (b != 0)
    {
        temp = a % b;

        a = b;
        b = temp;
    }
    return a;
}

void profile(void (*func)(void))
{
    clock_t t0, t1;
    int ms;
    int gcd = euclid(MS_PER_SEC, CLOCKS_PER_SEC);

    printf("----------------------------\n");
    printf("profile:\n");
    // printf("clock gcd = %d\n", gcd);

    t0 = clock();
    func();
    t1 = clock();

    ms = t1 - t0;
    ms *= (MS_PER_SEC / gcd);
    ms /= (CLOCKS_PER_SEC / gcd);

    printf("ms = %d\n", ms);
    printf("----------------------------\n");
}
