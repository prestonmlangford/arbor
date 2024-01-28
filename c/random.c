#include <stdint.h>
#include <time.h>
#include "random.h"

#define DEFAULT_STATE_V 0x12345678U

uint32_t state_u = 0U;
uint32_t state_v = DEFAULT_STATE_V;

#define ROR(u,s) (((u) >> (s)) | ((u) << (32 - (s))))
#define ROL(u,s) (((u) << (s)) | ((u) >> (32 - (s))))

static uint32_t xorshift(void)
{
    uint32_t v = state_v;

    v ^= v << 13;
    v ^= v >> 17;
    v ^= v << 5;

    state_v = v;

    return v;
}

static uint32_t nextpow2(uint32_t u)
{
    uint32_t v = u;

    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;

    return v;
}

void rand_seed(int seed)
{
    state_u = 0U;

    if (seed > 0)
    {
        state_v = (uint32_t) seed;
    }
    else if (seed < 0)
    {
        state_v = (uint32_t) (-seed);
    }
    else
    {
        state_v = DEFAULT_STATE_V;
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
        uint32_t u = xorshift();
        r = (int) (u & mask);
    } while (r >= range);

    return r + lower;
}
