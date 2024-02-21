#include <stdio.h>
#include "arbor.h"
#include "random.h"

int main (int argc, char* argv[])
{

    Arbor_Game game = arbor_new();

    rand_seed_realtime();

    printf("Arbor - Bad Battleship\n");

    while (arbor_side(game) != ARBOR_NONE)
    {
        if (arbor_side(game) == ARBOR_P1)
        {
            Arbor_Search_Config cfg = {
                .expansion = 10,
                .exploration = 2.0,
                .init = game,
                .eval_policy = ARBOR_EVAL_CUSTOM,
                .size = 1024 * 1024
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
            arbor_make(game, 0);
        }
    }

    printf("\nGame Over!\n");
    arbor_delete(game);

    return 0;
}