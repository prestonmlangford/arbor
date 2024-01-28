#include <assert.h>
#include "arbor.h"
#include "dice.h"
#include "random.h"

#define ABS(i) (((i) < 0) ? -(i) : (i))
#define DICE_LIMIT 21

typedef struct Dice_t
{
    int p1;
    int p2;
    int side;
    int result;
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

        if (dice->p1 >= DICE_LIMIT)
        {
            dice->side = ARBOR_NONE;
            dice->result = ARBOR_P2;
        }
        else
        {
            dice->side = ARBOR_P2;
        }
    }
    else
    {
        dice->p2 += score;

        if (dice->p2 >= DICE_LIMIT)
        {
            dice->side = ARBOR_NONE;
            dice->result = ARBOR_P1;
        }
        else
        {
            dice->side = ARBOR_P1;
        }
    }
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
        int p1 = DICE_LIMIT - dice->p2;
        int p2 = DICE_LIMIT - dice->p1;
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
}

