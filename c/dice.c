#include <assert.h>
#include "arbor.h"
#include "dice.h"
#include "random.h"

#define ABS(i) (((i) < 0) ? -(i) : (i))
#define NUM_TURNS 10

typedef struct Dice_t
{
    int p1;
    int p2;
    int side;
    int result;
    int turn;
} Dice;

Arbor_Game dice_new(void)
{
    Dice* dice = malloc(sizeof(Dice));

    dice->p1 = 0;
    dice->p2 = 0;
    dice->side = ARBOR_P1;
    dice->result = ARBOR_NONE;

    return (Arbor_Game) {dice};
}

void dice_make(Arbor_Game game, int action)
{
    Dice* dice = game.p;
    int roll_1 = rand_range(1,7);
    int roll_2 = rand_range(1,7);
    int sum = roll_1 + roll_2;
    int score = ABS(sum - action);

    if (dice->side == ARBOR_P1)
    {
        dice->p1 += score;
        dice->side = ARBOR_P2;
    }
    else
    {
        dice->p2 += score;
        dice->side = ARBOR_P1;
    }

    if (dice->turn >= NUM_TURNS)
    {
        if (dice->p1 == dice->p2)
        {
            dice->result = ARBOR_DRAW;
        }
        if (dice->p1 > dice->p2)
        {
            dice->result = ARBOR_P2;
        }
        else
        {
            dice->result = ARBOR_P1;
        }

        dice->side = ARBOR_NONE;
    }

    dice->turn += 1;
}

Arbor_Game dice_copy(Arbor_Game game)
{
    Dice* p = malloc(sizeof(Dice));
    Dice* dice = game.p;
    Arbor_Game copy = {p};

    *p = *dice;

    return copy;
}

void dice_delete(Arbor_Game game)
{
    free(game.p);
}

int dice_actions(Arbor_Game game)
{
    return 11;
}

int dice_side(Arbor_Game game)
{
    Dice* dice = game.p;

    return dice->side;
}

int dice_eval(Arbor_Game game)
{
    Dice* dice = game.p;

    if (dice->side == ARBOR_NONE)
    {
        return dice->result;
    }
    else
    {
        return ARBOR_DRAW;
    }
}

/*
 2: 1-1
 3: 1-2, 2-1,
 4: 1-3, 2-2, 3-1
 5: 1-4, 2-3, 3-2, 4-1
 6: 1-5, 2-4, 3-3, 4-2, 5-1
 7: 1-6, 2-5, 3-4, 4-3, 5-2, 6-1
 8: 2-6, 3-5, 4-4, 5-3, 6-2
 9: 3-6, 4-5, 5-4, 6-3
10: 4-6, 5-5, 6-4
11: 5-6, 6-5
12: 6-6
*/