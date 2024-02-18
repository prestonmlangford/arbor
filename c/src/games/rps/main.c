#include <stdio.h>
#include "arbor.h"
#include "random.h"

int main (int argc, char* argv[])
{

    Arbor_Game game = arbor_new();

    rand_seed_realtime();

    printf("Arbor - Rock, Paper, Scissors\n");

    while (arbor_side(game) != ARBOR_NONE)
    {
        int p1_action = 0;
        if (arbor_side(game) == ARBOR_P2)
        {
            Arbor_Search_Config cfg = {
                .expansion = 10,
                .exploration = 2.0,
                .init = game,
                .eval_policy = ARBOR_EVAL_ROLLOUT
            };

            Arbor_Search search = arbor_search_new(&cfg);
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
            arbor_make(game, best);
        }
        else
        {
            p1_action = (p1_action + 1) % arbor_actions(game);
            // p1_action = 0;
            arbor_make(game, p1_action);
        }
    }

    printf("\nGame Over!\n");
    arbor_delete(game);

    return 0;
}
