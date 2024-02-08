#include <assert.h>
#include "arbor.h"
#include "rps.h"
#include "random.h"

#define ABS(i) (((i) < 0) ? -(i) : (i))
#define NUM_TURNS 10

enum
{
    ROCK,
    PAPER,
    SCISSORS,
    ACTIONS,
};

int rules[ACTIONS] = {PAPER, SCISSORS, ROCK};

typedef struct RPS_t
{
    int p1;
    int p2;
    int last;
    int side;
    int turn;
} RPS;

Arbor_Game rps_new(void)
{
    RPS* rps = malloc(sizeof(RPS));

    rps->last = ROCK;
    rps->side = ARBOR_P1;

    return (Arbor_Game) {rps};
}

void rps_make(Arbor_Game game, int action)
{
    RPS* rps = game.p;
    int score = 0;
    int winner = rules[rps->last];

    if (action == rps->last)
    {
        score = 0;
    }
    else if (action == winner)
    {
        score = 1;
    }
    else
    {
        score = -1;
    }

    if (rps->side == ARBOR_P1)
    {
        rps->p1 += score;
        rps->side = ARBOR_P2;
    }
    else
    {
        rps->p2 += score;
        rps->side = ARBOR_P1;
    }

    rps->turn += 1;
    rps->last = action;

    if (rps->turn >= NUM_TURNS)
    {
        rps->side = ARBOR_NONE;
    }
}

Arbor_Game rps_copy(Arbor_Game game)
{
    RPS* p = malloc(sizeof(RPS));
    RPS* t = game.p;
    Arbor_Game copy = {p};

    *p = *t;

    return copy;
}

void rps_delete(Arbor_Game game)
{
    free(game.p);
}

int rps_actions(Arbor_Game game)
{
    return ACTIONS;
}

int rps_side(Arbor_Game game)
{
    RPS* rps = game.p;

    return rps->side;
}

int rps_eval(Arbor_Game game)
{
    RPS* rps = game.p;

    if (rps->side == ARBOR_NONE)
    {
        if (rps->p1 == rps->p2)
        {
            return ARBOR_DRAW;
        }
        else if (rps->p1 > rps->p2)
        {
            return ARBOR_P1;
        }
        else
        {
            return ARBOR_P2;
        }
    }
    else
    {
        return ARBOR_DRAW;
    }
}

