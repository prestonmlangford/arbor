#include <assert.h>
#include "arbor.h"
#include "bad_battleship.h"
#include "random.h"

#define NUM_PINS 16

typedef struct Bad_Battleship_t
{
    uint16_t p1_pins;
    uint16_t p2_pins;
    int side;
} BB;


static int bitcount(uint16_t u)
{
    int sum = 0;
    
    while (u > 0U)
    {
        sum += 1U;
        u &= u - 1U;
    }

    return sum;
}

Arbor_Game bb_new(void)
{
    BB* bb = malloc(sizeof(BB));

    bb->p1_pins = 0;
    bb->p2_pins = 0;
    bb->side = ARBOR_P1;

    return (Arbor_Game) {bb};
}

void bb_make(Arbor_Game game, int action)
{
    BB* bb = game.p;
    uint16_t pin = 1U << action;

    assert(bb != NULL);
    assert(bb->side   != ARBOR_NONE);
    assert(action < NUM_PINS);

    if (bb->side == ARBOR_P1)
    {
        bb->p2_pins |= pin;
        bb->side = ARBOR_P2;
    }
    else
    {
        bb->p1_pins |= pin;
        bb->side = ARBOR_P1;
    }

    if (bb->p1_pins == UINT16_MAX)
    {
        bb->side = ARBOR_NONE;
    }

    if (bb->p2_pins == UINT16_MAX)
    {
        bb->side = ARBOR_NONE;
    }
}

Arbor_Game bb_copy(Arbor_Game game)
{
    BB* p = malloc(sizeof(BB));
    BB* bb = game.p;

    *p = *bb;

    return (Arbor_Game){p};
}

void bb_delete(Arbor_Game game)
{
    free(game.p);
}

int bb_actions(Arbor_Game game)
{
    return NUM_PINS;
}

int bb_side(Arbor_Game game)
{
    BB* bb = game.p;

    return bb->side;
}

int bb_eval(Arbor_Game game)
{
    BB* bb = game.p;
    uint16_t p1 = bb->p1_pins;
    uint16_t p2 = bb->p1_pins;

    if (bb->side == ARBOR_NONE)
    {
        if (bb->p1_pins == bb->p2_pins)
        {
            return ARBOR_DRAW;
        }
        else if (bb->p1_pins < bb->p2_pins)
        {
            return ARBOR_P1;
        }
        else
        {
            return ARBOR_P2;
        }
    }
    else if (true)
    {
        int p1 = 16 - bitcount(bb->p2_pins);
        int p2 = 16 - bitcount(bb->p1_pins);
        int r = rand_range(0, p1 + p2);

        if (r < p1)
        {
            return ARBOR_P2;
        }
        else
        {
            return ARBOR_P1;
        }
    }
    else
    {
        if (bb->side == ARBOR_P1)
        {
            if (bb->p1_pins <= bb->p2_pins)
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
            if (bb->p2_pins <= bb->p1_pins)
            {
                return ARBOR_P2;
            }
            else
            {
                return ARBOR_P1;
            }
        }
    }
}

