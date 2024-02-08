#include <stdio.h>
#include "arbor.h"
#include "random.h"
#include "rps.h"

int main (int argc, char* argv[])
{

    Arbor_Game_Interface ifc = {
        .actions = rps_actions,
        .copy = rps_copy,
        .delete = rps_delete,
        .make = rps_make,
        .eval = rps_eval,
        .side = rps_side
    };

    Arbor_Game game = rps_new();

    rand_seed_realtime();

    printf("Arbor - Rock, Paper, Scissors\n");

    while (rps_side(game) != ARBOR_NONE)
    {
        int p1_action = 0;
        if (rps_side(game) == ARBOR_P2)
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
            rps_make(game, best);
        }
        else
        {
            p1_action = (p1_action + 1) % rps_actions(game);
            // p1_action = 0;
            rps_make(game, p1_action);
        }
    }

    printf("\nGame Over!\n");
    rps_delete(game);

    return 0;
}
