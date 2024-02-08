#include <stdio.h>
#include "arbor.h"
#include "dice.h"
#include "random.h"

int main (int argc, char* argv[])
{

    Arbor_Game_Interface ifc = {
        .actions = dice_actions,
        .copy = dice_copy,
        .delete = dice_delete,
        .make = dice_make,
        .eval = dice_eval,
        .side = dice_side
    };

    Arbor_Game game = dice_new();

    rand_seed_realtime();

    printf("Arbor - Dice 21\n");

    while (dice_side(game) != ARBOR_NONE)
    {
        if (dice_side(game) == ARBOR_P1)
        {
            Arbor_Search_Config cfg = {
                .expansion = 10,
                .exploration = 2.0,
                .init = game,
                .eval_policy = ARBOR_EVAL_ROLLOUT
            };

            Arbor_Search search = arbor_search_new(&cfg, &ifc);
            int i = 0;
            int best = 0;

            for (i = 0; i < 100000; i++)
            {
                arbor_search_ponder(search);
            }

            best = arbor_search_best(search);
            arbor_search_delete(search);

            printf("%d ",best);
            fflush(stdout);
            dice_make(game, best);
        }
        else
        {
            dice_make(game, 0);
        }
    }

    printf("\nGame Over!\n");
    dice_delete(game);

    return 0;
}
