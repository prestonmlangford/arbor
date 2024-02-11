#include <stdio.h>
#include <stdint.h>
#include <assert.h>
#include "arbor.h"
#include "reversi.h"
#include "random.h"

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
    uint64_t x = 0;\
    x |= op(f) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    op(x) & ~(f | e);\
})

#define CAPTURE(p,f,e,op)({\
    uint64_t x = p;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    x |= op(x) & e;\
    MASK(op(x) & f) & x;\
})

typedef struct Reversi_t
{
    uint64_t f;
    uint64_t e;
    uint64_t a;
    int side;
    bool pass;
    int result;
} Reversi;

inline static int popcount(uint64_t u)
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

inline static uint64_t generate_moves(uint64_t f, uint64_t e)
{
    uint64_t u = 0;

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

inline static uint64_t make_capture(uint64_t f, uint64_t e, uint64_t c)
{
    uint64_t u = 0;

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

inline static uint64_t mobility(uint64_t f, uint64_t e)
{
    uint64_t u = 0;
    uint64_t a = ~(f | e);

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

Arbor_Game reversi_new(void)
{
    Reversi* rev = malloc(sizeof(Reversi));

    *rev = (Reversi) {
        .f = (UINT64_C(1) << 043) | (UINT64_C(1) << 034),
        .e = (UINT64_C(1) << 033) | (UINT64_C(1) << 044),
        .side = ARBOR_P1,
        .pass = false,
        .result = ARBOR_NONE
    };

    rev->a = generate_moves(rev->f, rev->e);

    return (Arbor_Game) {rev};
}

void reversi_make(Arbor_Game game, int action)
{
    Reversi* rev = game.p;
    uint64_t f = rev->f;
    uint64_t e = rev->e;
    uint64_t u = rev->a;
    bool gameover = false;

    if (u > 0)
    {
        uint64_t c = 0;
        int i = 0;

        while (i < action)
        {
            u &= u - 1;
            i++;
        }
        u &= ~(u - 1);

        c = make_capture(f, e, u);

        e &= ~c;
        f |=  c;

        rev->pass = false;
    }
    else if (rev->pass)
    {
        gameover = true;
    }
    else if ((f | e) == FULLBOARD)
    {
        gameover = true;
    }
    else
    {
        rev->pass = true;
    }

    rev->f = e;
    rev->e = f;

    if (gameover)
    {
        int sum_p1 = 0;
        int sum_p2 = 0;

        if (rev->side == ARBOR_P1)
        {
            sum_p1 = popcount(f);
            sum_p2 = popcount(e);
        }
        else
        {
            sum_p1 = popcount(e);
            sum_p2 = popcount(f);
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
        rev->a = generate_moves(rev->f, rev->e);
        
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
    int sum = popcount(rev->a);

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
    Reversi* reversi = game.p;

    return reversi->result;
}

void reversi_show(Arbor_Game game)
{
    Reversi* rev = game.p;
    const char* colnum = "    0   1   2   3   4   5   6   7\n";
    const char* rowsep = "  ---------------------------------\n";
    uint64_t moves = rev->a;
    uint64_t white = 0;
    uint64_t black = 0;
    int row = 0;
    int col = 0;

    if (rev->side == ARBOR_P1)
    {
        white = rev->f;
        black = rev->e;
        printf("O");
    }
    else
    {
        white = rev->e;
        black = rev->f;
        printf("X");
    }

    printf(" Turn\n");
    printf("O: %2d, X: %2d\n%s", popcount(white), popcount(black), rowsep);

    for (row = 7; row >= 0; row--)
    {
        printf("%d ", row);
        for (col = 0; col < 8; col++)
        {
            char p = ' ';
            uint64_t space = 1;

            space <<= (row << 3) | col;
            
            if (white & space)
            {
                p = 'O';
            }
            else if (black & space)
            {
                p = 'X';
            }
            else if (moves & space)
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

void reversi_heuristics(Arbor_Game game)
{
    Reversi* rev = game.p;
    uint64_t bb_features[] = {
        rev->f,
        rev->e,
        rev->a,
        generate_moves(rev->e, rev->f),
        mobility(rev->f, rev->e),
        mobility(rev->e, rev->f),
        rev->f & Q5,
        rev->f & Q4,
        rev->f & Q3,
        rev->f & Q2,
        rev->f & Q1,
        rev->e & Q5,
        rev->e & Q4,
        rev->e & Q3,
        rev->e & Q2,
        rev->e & Q1,
    };
    size_t num_features = sizeof(bb_features)/sizeof(uint64_t);
    size_t i = 0;

    for (i = 0; i < num_features; i++)
    {
        int f = popcount(bb_features[i]);
        printf("%d,", f);
    }
}
