#include <stdio.h>
#include <stdint.h>
#include <assert.h>
#include <math.h>
#include "arbor.h"
#include "reversi.h"
#include "reversi_coef.h"
#include "random.h"

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

#define PARITY(f,e)\
({\
    float _f = (float) bb_popcount(f);\
    float _e = (float) bb_popcount(e);\
    0.5*(1.0 + (_f - _e)/(_f + _e + __FLT_EPSILON__));\
})

#define PARITY_MASK(f,e,mask) PARITY((f) & (mask), (e) & (mask))

#define SW BB(0,0)
#define NW BB(7,0)
#define NE BB(7,7)
#define SE BB(0,7)
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

typedef struct Reversi_t
{
    bb f;
    bb e;
    bb a;
    int side;
    bool pass;
    int result;
    int turn;
} Reversi;

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


Arbor_Game reversi_new(void)
{
    Reversi* rev = malloc(sizeof(Reversi));

    *rev = (Reversi) {
        .f = BB(4,3) | BB(3,4),
        .e = BB(3,3) | BB(4,4),
        .side = ARBOR_P1,
        .pass = false,
        .result = ARBOR_NONE
    };

    rev->a = bb_generate_moves(rev->f, rev->e);

    return (Arbor_Game) {rev};
}

void reversi_make(Arbor_Game game, int action)
{
    Reversi* rev = game.p;
    bb f = rev->f;
    bb e = rev->e;
    bb u = rev->a;
    bool gameover = false;

    if (u > 0)
    {
        bb c = 0;
        int i = 0;

        while (i < action)
        {
            u &= u - 1;
            i++;
        }
        u &= ~(u - 1);

        c = bb_make_capture(f, e, u);

        e &= ~c;
        f |=  c;

        rev->pass = false;
    }
    else if (rev->pass)
    {
        gameover = true;
    }
    else if (~(f | e))
    {
        rev->pass = true;
    }
    else
    {
        gameover = true;
    }

    rev->f = e;
    rev->e = f;
    rev->turn += 1;

    if (gameover)
    {
        int sum_p1 = 0;
        int sum_p2 = 0;

        if (rev->side == ARBOR_P1)
        {
            sum_p1 = bb_popcount(f);
            sum_p2 = bb_popcount(e);
        }
        else
        {
            sum_p1 = bb_popcount(e);
            sum_p2 = bb_popcount(f);
        }

        if (sum_p1 > sum_p2)
        {
            rev->result = ARBOR_P1;
        }
        else if (sum_p2 > sum_p1)
        {
            rev->result = ARBOR_P2;
        }
        else
        {
            rev->result = ARBOR_DRAW;
        }

        rev->side = ARBOR_NONE;
    }
    else
    {
        rev->a = bb_generate_moves(rev->f, rev->e);
        
        if (rev->side == ARBOR_P1)
        {
            rev->side = ARBOR_P2;
        }
        else
        {
            rev->side = ARBOR_P1;
        }
    }
}

Arbor_Game reversi_copy(Arbor_Game game)
{
    Reversi* p = malloc(sizeof(Reversi));
    Reversi* t = game.p;
    Arbor_Game copy = {p};

    *p = *t;

    return copy;
}

void reversi_delete(Arbor_Game game)
{
    free(game.p);
}

int reversi_actions(Arbor_Game game)
{
    Reversi* rev = game.p;
    int sum = bb_popcount(rev->a);

    // add one for pass if no other option
    sum += (sum == 0);

    return sum;
}

int reversi_side(Arbor_Game game)
{
    Reversi* reversi = game.p;

    return reversi->side;
}

int reversi_eval(Arbor_Game game)
{
    Reversi* rev = game.p;

    if (rev->side == ARBOR_NONE)
    {
        return rev->result;
    }
    else
    {
        bb p1 = (rev->side == ARBOR_P1) ? rev->f : rev->e;
        bb p2 = (rev->side == ARBOR_P2) ? rev->f : rev->e;
        int i = 0;
        float sum = 0.0;
        float feat[NUM_FEAT] = {};
        float* coef = reversi_heuristic_coef[rev->turn];

        bb_vector(p1, p2, feat);

        for (i = 0; i < NUM_FEAT; i++)
        {
            sum += feat[i] * coef[i];
        }
        sum += coef[i];
        
        if (sum > 0.0)
        {
            return ARBOR_P1;
        }
        else
        {
            return ARBOR_P2;
        }
    }
}

void reversi_show(Arbor_Game game)
{
    Reversi* rev = game.p;
    bb moves = rev->a;
    bb X = 0;
    bb O = 0;

    if (rev->side == ARBOR_P1)
    {
        O = rev->f;
        X = rev->e;
        printf("O");
    }
    else
    {
        O = rev->e;
        X = rev->f;
        printf("X");
    }

    printf(" Turn\n");
    printf("O: %2d, X: %2d\n", bb_popcount(O), bb_popcount(X));

    bb_show(X, O, moves);
}

void reversi_vector(Arbor_Game game)
{
    Reversi* rev = game.p;
    bb p1 = (rev->side == ARBOR_P1) ? rev->f : rev->e;
    bb p2 = (rev->side == ARBOR_P2) ? rev->f : rev->e;

    float features[NUM_FEAT] = {};
    int last_feature = NUM_FEAT - 1;
    int i = 0;

    bb_vector(p1, p2, features);

    for (i = 0; i < NUM_FEAT; i++)
    {
        char sep = (i == last_feature) ? '\n' : ',';
        printf("%f%c", features[i], sep);
    }
}

void reversi_prob(Arbor_Game game)
{
    Reversi* rev = game.p;
    bb p1 = (rev->side == ARBOR_P1) ? rev->f : rev->e;
    bb p2 = (rev->side == ARBOR_P2) ? rev->f : rev->e;
    int i = 0;
    float sum = 0.0;
    float feat[NUM_FEAT] = {};
    float* coef = reversi_heuristic_coef[rev->turn];
    float p = 0.5;

    bb_vector(p1, p2, feat);

    for (i = 0; i < NUM_FEAT; i++)
    {
        sum += feat[i] * coef[i];
    }
    sum += coef[i];

    p = 1.0 / (1.0 + exp(-sum));

    printf("%f\n",p);
}
