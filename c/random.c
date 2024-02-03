#include <stdint.h>
#include <time.h>
#include "random.h"

#define DEFAULT_STATE_V 0x12345678U

uint32_t random_state = DEFAULT_STATE_V;

#define ROR(u,s) (((u) >> (s)) | ((u) << (32 - (s))))
#define ROL(u,s) (((u) << (s)) | ((u) >> (32 - (s))))

inline static uint32_t xorshift(uint32_t u)
{
    u ^= u << 13;
    u ^= u >> 17;
    u ^= u << 5;

    return u;
}

static uint32_t nextpow2(uint32_t u)
{
#if 0
    return UINT32_C(1) << (32 - __builtin_clz (u - UINT32_C(1)));
#else
    u |= u >> 1;
    u |= u >> 2;
    u |= u >> 4;
    u |= u >> 8;
    u |= u >> 16;

    return u;
#endif
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
#ifdef USE_REJECTION_SAMPLING
    int max = upper - lower - 1;
    uint32_t mask = nextpow2(max);
    int r = 0;

    do
    {
        random_state = xorshift(random_state);
        r = (int) (random_state & mask);
    } while (r > max);

    return r + lower;
#else
    uint32_t range = (uint32_t) (upper - lower);
    int r = 0;

    random_state = xorshift(random_state);

    // the use case for arbor has insignificant
    // modulo bias since random state >> range
    r = (int) (random_state % range);

    return r + lower;
#endif
}
