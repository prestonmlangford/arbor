#include <stdio.h>
#include <stdint.h>
#include <assert.h>
#include "arbor.h"
#include "reversi.h"
#include "reversi_bb.h"
#include "random.h"

typedef struct Reversi_t
{
    bb f;
    bb e;
    bb a;
    int side;
    bool pass;
    int result;
} Reversi;


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
    Reversi* reversi = game.p;

    return reversi->result;
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

    bb_vector(p1, p2);
}
