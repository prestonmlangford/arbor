#include <stdio.h>
#include "reversi_bb.h"

/*
Q5
  ---------------------------------
7 | x |   |   |   |   |   |   | x |
  ---------------------------------
6 |   |   |   |   |   |   |   |   |
  ---------------------------------
5 |   |   |   |   |   |   |   |   |
  ---------------------------------
4 |   |   |   |   |   |   |   |   |
  ---------------------------------
3 |   |   |   |   |   |   |   |   |
  ---------------------------------
2 |   |   |   |   |   |   |   |   |
  ---------------------------------
1 |   |   |   |   |   |   |   |   |
  ---------------------------------
0 | x |   |   |   |   |   |   | x |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/
#define Q5 UINT64_C(0x8100000000000081)

/*
Q4
  ---------------------------------
7 |   | x |   |   |   |   | x |   |
  ---------------------------------
6 | x |   |   |   |   |   |   | x |
  ---------------------------------
5 |   |   |   |   |   |   |   |   |
  ---------------------------------
4 |   |   |   |   |   |   |   |   |
  ---------------------------------
3 |   |   |   |   |   |   |   |   |
  ---------------------------------
2 |   |   |   |   |   |   |   |   |
  ---------------------------------
1 | x |   |   |   |   |   |   | x |
  ---------------------------------
0 |   | x |   |   |   |   | x |   |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/
#define Q4 UINT64_C(0x4281000000008142)

/*
Q3
  ---------------------------------
7 |   |   |   |   |   |   |   |   |
  ---------------------------------
6 |   | x |   |   |   |   | x |   |
  ---------------------------------
5 |   |   |   |   |   |   |   |   |
  ---------------------------------
4 |   |   |   |   |   |   |   |   |
  ---------------------------------
3 |   |   |   |   |   |   |   |   |
  ---------------------------------
2 |   |   |   |   |   |   |   |   |
  ---------------------------------
1 |   | x |   |   |   |   | x |   |
  ---------------------------------
0 |   |   |   |   |   |   |   |   |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/
#define Q3 UINT64_C(0x0042000000004200)

/*
Q2
  ---------------------------------
7 |   |   | x | x | x | x |   |   |
  ---------------------------------
6 |   |   |   |   |   |   |   |   |
  ---------------------------------
5 | x |   |   |   |   |   |   | x |
  ---------------------------------
4 | x |   |   |   |   |   |   | x |
  ---------------------------------
3 | x |   |   |   |   |   |   | x |
  ---------------------------------
2 | x |   |   |   |   |   |   | x |
  ---------------------------------
1 |   |   |   |   |   |   |   |   |
  ---------------------------------
0 |   |   | x | x | x | x |   |   |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/
#define Q2 UINT64_C(0x3C8181818181813C)

/*
Q1
  ---------------------------------
7 |   |   |   |   |   |   |   |   |
  ---------------------------------
6 |   |   | x | x | x | x |   |   |
  ---------------------------------
5 |   | x |   |   |   |   | x |   |
  ---------------------------------
4 |   | x |   |   |   |   | x |   |
  ---------------------------------
3 |   | x |   |   |   |   | x |   |
  ---------------------------------
2 |   | x |   |   |   |   | x |   |
  ---------------------------------
1 |   |   | x | x | x | x |   |   |
  ---------------------------------
0 |   |   |   |   |   |   |   |   |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/
#define Q1 UINT64_C(0x003C424242423C00)

/*
Q0
  ---------------------------------
7 |   |   |   |   |   |   |   |   |
  ---------------------------------
6 |   |   |   |   |   |   |   |   |
  ---------------------------------
5 |   |   | x |   |   | x |   |   |
  ---------------------------------
4 |   |   |   | x | x |   |   |   |
  ---------------------------------
3 |   |   |   | x | x |   |   |   |
  ---------------------------------
2 |   |   | x |   |   | x |   |   |
  ---------------------------------
1 |   |   |   |   |   |   |   |   |
  ---------------------------------
0 |   |   |   |   |   |   |   |   |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/
#define Q0 UINT64_C(0x0000241818240000)

#define FULLBOARD UINT64_C(0xFFFFFFFFFFFFFFFF)
#define EASTBOUND UINT64_C(0x7F7F7F7F7F7F7F7F)
#define WESTBOUND UINT64_C(0xFEFEFEFEFEFEFEFE)
#define NORTH(x)     (x << 8)
#define SOUTH(x)     (x >> 8)
#define EAST(x)      ((x << 1) & WESTBOUND)
#define WEST(x)      ((x >> 1) & EASTBOUND)
#define NORTHEAST(x) ((x << 9) & WESTBOUND)
#define NORTHWEST(x) ((x << 7) & EASTBOUND)
#define SOUTHEAST(x) ((x >> 7) & WESTBOUND)
#define SOUTHWEST(x) ((x >> 9) & EASTBOUND)
#define MASK(x) (((x) == UINT64_C(0)) - UINT64_C(1))

#define CHECK(f,e,op)({\
    bb x = 0;\
    x |= op(f) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    op(x) & ~(f | e);\
})

#define CAPTURE(p,f,e,op)({\
    bb x = p;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    MASK(op(x) & f) & x;\
})

#define PARITY(f,e)\
({\
    float _f = (float) bb_popcount(f);\
    float _e = (float) bb_popcount(e);\
    0.5*(1.0 + (_f - _e)/(_f + _e + __FLT_EPSILON__));\
})

int bb_popcount(bb u)
{
#if USE_BUILTINS
    return __builtin_popcountll(u);
#else
    int sum = 0;

    while (u > 0)
    {
        sum += 1;
        u &= u - 1;
    }

    return sum;
#endif
}

// 0 1 2 3 4 5 6 7
// - - W B B - - -
// 0 0 0 1 0 0 0 0
// 0 0 0 1 1 0 0 0
// 0 0 0 1 1 0 0 0
// 0 0 0 1 1 0 0 0
// 0 0 0 1 1 0 0 0
// 0 0 0 1 1 0 0 0
// 0 0 0 0 0 1 0 0


// 0 1 2 3 4 5 6 7
// W B B B B B B -
// 0 1 0 0 0 0 0 0
// 0 1 1 0 0 0 0 0
// 0 1 1 1 0 0 0 0
// 0 1 1 1 1 0 0 0
// 0 1 1 1 1 1 0 0
// 0 1 1 1 1 1 1 0
// 0 0 0 0 0 0 0 1

// https://www.gamedev.net/forums/topic/646988-generating-moves-in-reversi/

bb bb_generate_moves(bb f, bb e)
{
    bb u = 0;

    u |= CHECK(f,e,NORTH);
    u |= CHECK(f,e,SOUTH);
    u |= CHECK(f,e,EAST);
    u |= CHECK(f,e,WEST);
    u |= CHECK(f,e,NORTHEAST);
    u |= CHECK(f,e,NORTHWEST);
    u |= CHECK(f,e,SOUTHEAST);
    u |= CHECK(f,e,SOUTHWEST);

    return u;
}

bb bb_make_capture(bb f, bb e, bb c)
{
    bb u = 0;

    u |= CAPTURE(c,f,e,NORTH);
    u |= CAPTURE(c,f,e,SOUTH);
    u |= CAPTURE(c,f,e,EAST);
    u |= CAPTURE(c,f,e,WEST);
    u |= CAPTURE(c,f,e,NORTHEAST);
    u |= CAPTURE(c,f,e,NORTHWEST);
    u |= CAPTURE(c,f,e,SOUTHEAST);
    u |= CAPTURE(c,f,e,SOUTHWEST);

    return u;
}

bb bb_mobility(bb f, bb e)
{
    bb u = 0;

    u |= NORTH(e);
    u |= SOUTH(e);
    u |= EAST(e);
    u |= WEST(e);
    u |= NORTHEAST(e);
    u |= NORTHWEST(e);
    u |= SOUTHEAST(e);
    u |= SOUTHWEST(e);

    return u & ~(f | e);
}

void bb_vector(bb f, bb e)
{
    float features[] =
    {
        PARITY(f, e), 
        PARITY(bb_generate_moves(f, e), bb_generate_moves(e, f)),
        PARITY(bb_mobility(f, e), bb_mobility(e, f)),
        PARITY(f & Q5, e & Q5),
        PARITY(f & Q4, e & Q4),
        PARITY(f & Q3, e & Q3),
        PARITY(f & Q2, e & Q2),
        PARITY(f & Q1, e & Q1),
        PARITY(f & Q0, e & Q0)
    };
    int num_features = sizeof(features)/sizeof(float);
    int last_feature = num_features - 1;
    int i = 0;

    for (i = 0; i < num_features; i++)
    {
        char sep = (i == last_feature) ? '\n' : ',';
        printf("%f%c", features[i], sep);
    }
}
