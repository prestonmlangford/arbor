#include <stdint.h>
#include <time.h>
#include "random.h"

#define DEFAULT_STATE_V 0x12345678U

uint32_t random_state = DEFAULT_STATE_V;

#define ROR(u,s) (((u) >> (s)) | ((u) << (32 - (s))))
#define ROL(u,s) (((u) << (s)) | ((u) >> (32 - (s))))

static uint32_t xorshift(uint32_t u)
{
    u ^= u << 13;
    u ^= u >> 17;
    u ^= u << 5;

    return u;
}

static uint32_t nextpow2(uint32_t u)
{
    u |= u >> 1;
    u |= u >> 2;
    u |= u >> 4;
    u |= u >> 8;
    u |= u >> 16;

    return u;
}

void rand_seed(int seed)
{
    if (seed > 0)
    {
        random_state = (uint32_t) seed;
    }
    else if (seed < 0)
    {
        random_state = (uint32_t) (-seed);
    }
    else
    {
        random_state = DEFAULT_STATE_V;
    }
}

void rand_seed_realtime(void)
{
    struct timespec ts = {};

    clock_gettime(CLOCK_REALTIME, &ts);

    rand_seed(ts.tv_nsec);
}

int rand_range(int lower, int upper)
{
    int range = upper - lower;
    uint32_t mask = nextpow2(range);
    int r = 0;

    do
    {
        random_state = xorshift(random_state);
        r = (int) (random_state & mask);
    } while (r >= range);

    return r + lower;
}
