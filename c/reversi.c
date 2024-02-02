#include <stdio.h>
#include <stdint.h>
#include <assert.h>
#include "arbor.h"
#include "reversi.h"
#include "random.h"

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
    int side;
    bool pass;
    int result;
} Reversi;

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

static int popcount(uint64_t u)
{
    int sum = 0;

    while (u > 0)
    {
        sum += 1;
        u &= u - 1;
    }

    return sum;
}

//https://www.gamedev.net/forums/topic/646988-generating-moves-in-reversi/
static uint64_t parallel_capture(uint64_t f, uint64_t e)
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

    return (Arbor_Game) {rev};
}

void reversi_make(Arbor_Game game, int action)
{
    Reversi* rev = game.p;
    uint64_t f = rev->f;
    uint64_t e = rev->e;
    uint64_t u = parallel_capture(f, e);
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

        c |= CAPTURE(u,f,e,NORTH);
        c |= CAPTURE(u,f,e,SOUTH);
        c |= CAPTURE(u,f,e,EAST);
        c |= CAPTURE(u,f,e,WEST);
        c |= CAPTURE(u,f,e,NORTHEAST);
        c |= CAPTURE(u,f,e,NORTHWEST);
        c |= CAPTURE(u,f,e,SOUTHEAST);
        c |= CAPTURE(u,f,e,SOUTHWEST);

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
    else if (rev->side == ARBOR_P1)
    {
        rev->side = ARBOR_P2;
    }
    else
    {
        rev->side = ARBOR_P1;
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
    uint64_t u = parallel_capture(rev->f, rev->e);
    int sum = popcount(u);

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
    uint64_t moves = parallel_capture(rev->f, rev->e);
    uint64_t white = 0;
    uint64_t black = 0;
    int row = 0;
    int col = 0;

    if (rev->side == ARBOR_P1)
    {
        white = rev->f;
        black = rev->e;
        printf("White");
    }
    else
    {
        white = rev->e;
        black = rev->f;
        printf("Black");
    }

    printf(" Turn\n");
    printf("White: %d, Black: %d\n%s", popcount(white), popcount(black), rowsep);

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
                p = 'W';
            }
            else if (black & space)
            {
                p = 'B';
            }
            else if (moves & space)
            {
                p = 'x';
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

void bin(uint64_t u)
{
    int i;
    for (i = 63; i >= 0; i--)
    {
        printf("%d",(int)((u >> i) & 1));
    }
    printf("\n");
}

int reversi_convert_xy(Arbor_Game game, int xy)
{
    Reversi* rev = game.p;
    int action;
    uint64_t a,u,v,p;

    u = parallel_capture(rev->f, rev->e);
    action = 0;
    p = 1;
    p <<= xy;

    while (u > 0)
    {
        v = u - 1;
        a = u & ~v;
        u = u & v;

        if (a == p)
        {
            break;
        }

        action++;
    }

    return action;   
}

int reversi_convert_action(Arbor_Game game, int action)
{
    Reversi* rev = game.p;
    uint64_t a,u,v,p;
    int i = 0;

    u = parallel_capture(rev->f, rev->e);

    if (u == 0)
    {
        return -1;
    }

    while (u > 0)
    {
        v = u - 1;
        a = u & ~v;
        u = u & v;

        if (i == action)
        {
            break;
        }

        i++;
    }

    return popcount(a - 1);   
}
