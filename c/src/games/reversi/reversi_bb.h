#ifndef REVERSI_BB_H
#define REVERSI_BB_H

#include <stdint.h>
#include <stdio.h>
#include "reversi_coef.h"

#define BB(y,x) (UINT64_C(1) << (((y) << 3) | (x)))

/*
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
#define CORNERS (BB(0,0) | BB(0,7) | BB(7,0) | BB(7,7))

/*
  ---------------------------------
7 |   | x |   |   |   |   | x |   |
  ---------------------------------
6 | x | x |   |   |   |   | x | x |
  ---------------------------------
5 |   |   |   |   |   |   |   |   |
  ---------------------------------
4 |   |   |   |   |   |   |   |   |
  ---------------------------------
3 |   |   |   |   |   |   |   |   |
  ---------------------------------
2 |   |   |   |   |   |   |   |   |
  ---------------------------------
1 | x | x |   |   |   |   | x | x |
  ---------------------------------
0 |   | x |   |   |   |   | x |   |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/
#define CORNERS_ADJACENT \
(\
    BB(1,0) | BB(1,1) | BB(0,1) |\
    BB(6,0) | BB(6,1) | BB(7,1) |\
    BB(6,7) | BB(6,6) | BB(7,6) |\
    BB(1,7) | BB(1,6) | BB(0,6)  \
)

/*
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
#define OUTSIDES \
(\
    BB(7,2) | BB(7,3) | BB(7,4) | BB(7,5) |\
    BB(2,0) | BB(3,0) | BB(4,0) | BB(5,0) |\
    BB(2,7) | BB(3,7) | BB(4,7) | BB(5,7) |\
    BB(0,2) | BB(0,3) | BB(0,4) | BB(0,5)  \
) 

/*
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
#define INSIDES \
(\
    BB(6,2) | BB(6,3) | BB(6,4) | BB(6,5) |\
    BB(2,1) | BB(3,1) | BB(4,1) | BB(5,1) |\
    BB(2,2) | BB(3,2) | BB(4,2) | BB(5,2) |\
    BB(1,2) | BB(1,3) | BB(1,4) | BB(1,5)  \
)

/*
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
#define DIAGONALS \
(\
    BB(5,2) | BB(5,5) | BB(2,2) | BB(2,5) |\
    BB(4,3) | BB(4,4) | BB(3,3) | BB(3,4) \
)

#define SW BB(0,0)
#define NW BB(7,0)
#define NE BB(7,7)
#define SE BB(0,7)
#define FULLBOARD UINT64_C(0xFFFFFFFFFFFFFFFF)
#define EASTBOUND UINT64_C(0x7F7F7F7F7F7F7F7F)
#define WESTBOUND UINT64_C(0xFEFEFEFEFEFEFEFE)
#define PARITY(f,e)\
({\
    float _f = (float) bb_popcount(f);\
    float _e = (float) bb_popcount(e);\
    0.5*(1.0 + (_f - _e)/(_f + _e + __FLT_EPSILON__));\
})

#define NORTH(x)     (x << 8)
#define SOUTH(x)     (x >> 8)
#define EAST(x)      ((x << 1) & WESTBOUND)
#define WEST(x)      ((x >> 1) & EASTBOUND)
#define NORTHEAST(x) ((x << 9) & WESTBOUND)
#define NORTHWEST(x) ((x << 7) & EASTBOUND)
#define SOUTHEAST(x) ((x >> 7) & WESTBOUND)
#define SOUTHWEST(x) ((x >> 9) & EASTBOUND)
#define MASK(x) (((x) == UINT64_C(0)) - UINT64_C(1))

#define CHECK(f,e,op)\
({\
    bb x = 0;\
    x |= op(f) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    op(x) & ~(f | e);\
})

#define CAPTURE(p,f,e,op)\
({\
    bb x = p;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    MASK(op(x) & f) & x;\
})

#define CONTROL(q,f,op)\
({\
    bb x = q;\
    x |= op(x) & f;\
    x |= op(x) & f;\
    x |= op(x) & f;\
    x |= op(x) & f;\
    x |= op(x) & f;\
    x |= op(x) & f;\
    x;\
})

typedef uint64_t bb;

inline static int bb_popcount(bb u)
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

inline static bb bb_generate_moves(bb f, bb e)
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

inline static bb bb_make_capture(bb f, bb e, bb c)
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

inline static bb bb_mobility(bb f, bb e)
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

/*
  ---------------------------------
7 | x | x | x | x | x | x | x | x |
  ---------------------------------
6 | x | x |   |   |   |   | x | x |
  ---------------------------------
5 | x |   | x |   |   | x |   | x |
  ---------------------------------
4 | x |   |   | x | x |   |   | x |
  ---------------------------------
3 | x |   |   | x | x |   |   | x |
  ---------------------------------
2 | x |   | x |   |   | x |   | x |
  ---------------------------------
1 | x | x |   |   |   |   | x | x |
  ---------------------------------
0 | x | x | x | x | x | x | x | x |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/
inline static bb bb_corner_stability(bb f)
{
    bb u = 0;
    bb sw = SW & f;
    bb nw = NW & f;
    bb ne = NE & f;
    bb se = SE & f;

    if (sw)
    {
        u |= CONTROL(sw, f, NORTH);
        u |= CONTROL(sw, f, EAST);
        u |= CONTROL(sw, f, NORTHEAST);
    }

    if (nw)
    {
        u |= CONTROL(nw, f, SOUTH);
        u |= CONTROL(nw, f, EAST);
        u |= CONTROL(nw, f, SOUTHEAST);
    }

    if (ne)
    {
        u |= CONTROL(ne, f, SOUTH);
        u |= CONTROL(ne, f, WEST);
        u |= CONTROL(ne, f, SOUTHWEST);
    }

    if (se)
    {
        u |= CONTROL(se, f, NORTH);
        u |= CONTROL(se, f, WEST);
        u |= CONTROL(se, f, NORTHWEST);
    }

    return u;
}

/*
  ---------------------------------
7 |   | x |   |   |   |   | x |   |
  ---------------------------------
6 | x | x |   |   |   |   | x | x |
  ---------------------------------
5 |   |   |   |   |   |   |   |   |
  ---------------------------------
4 |   |   |   |   |   |   |   |   |
  ---------------------------------
3 |   |   |   |   |   |   |   |   |
  ---------------------------------
2 |   |   |   |   |   |   |   |   |
  ---------------------------------
1 | x | x |   |   |   |   | x | x |
  ---------------------------------
0 |   | x |   |   |   |   | x |   |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/
inline static bb bb_corner_vulnerability(bb f)
{
    bb u = 0;
    bb c = (SW | NW | NE | SE) & ~f;
    
    u |= NORTH(c);
    u |= SOUTH(c);
    u |= EAST(c);
    u |= WEST(c);
    u |= NORTHEAST(c);
    u |= NORTHWEST(c);
    u |= SOUTHEAST(c);
    u |= SOUTHWEST(c);

    return u & f;
}

inline static void bb_vector(bb f, bb e, float v[NUM_FEAT])
{
    v[0] = PARITY(bb_mobility(f, e),           bb_mobility(e, f));
    v[1] = PARITY(bb_corner_stability(f),      bb_corner_stability(e));
    v[2] = PARITY(bb_corner_vulnerability(f),  bb_corner_vulnerability(e));
}

static void bb_show(bb x, bb o, bb d)
{
    const char* colnum = "    0   1   2   3   4   5   6   7\n";
    const char* rowsep = "  ---------------------------------\n";
    int row = 0;
    int col = 0;

    printf("%s", rowsep);

    for (row = 7; row >= 0; row--)
    {
        printf("%d ", row);
        for (col = 0; col < 8; col++)
        {
            char p = ' ';
            bb space = BB(row,col);
            
            if (o & space)
            {
                p = 'O';
            }
            else if (x & space)
            {
                p = 'X';
            }
            else if (d & space)
            {
                p = '-';
            }
            else
            {
                p = ' ';
            }
            printf("| %c ", p);
        }
        printf("|\n%s",rowsep);
    }
    printf("%s\n",colnum);
}

#endif // REVERSI_BB_H
