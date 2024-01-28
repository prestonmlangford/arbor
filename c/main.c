#include <time.h>
#include <stdio.h>
#include <stdint.h>
#include "arbor.h"
#include "random.h"
#include "bad_battleship.h"

uint32_t entropy(void)
{
    struct timespec ts = {};

    clock_gettime(CLOCK_REALTIME, &ts);

    return ts.tv_nsec;
}

int main (int argc, char* argv[])
{
    size_t KB = 1024;
    size_t MB = KB * KB;

    Arbor_Game_Interface ifc = {
        .actions = bb_actions,
        .copy = bb_copy,
        .free = bb_free,
        .make = bb_make,
        .eval = bb_eval,
        .side = bb_side
    };

    Arbor_Game game = bb_new();
    BB* bb = game.p;

    rand_seed_realtime();

    printf("Arbor - Bad Battleship\n");

    while (bb_side(game) != ARBOR_NONE)
    {
        if (bb_side(game) == ARBOR_P1)
        {
            Arbor_Search_Config cfg = {
                .expansion = 10,
                .exploration = 2.0,
                .size = 10U * MB,
                .init = game,
                .eval_policy = ARBOR_EVAL_CUSTOM
            };

            Arbor_Search search = arbor_search_new(&cfg, &ifc);
            int i = 0;
            int best = 0;

            for (i = 0; i < 100000; i++)
            {
                arbor_search_ponder(search);
            }

            best = arbor_search_best(search);
            printf("%d ",best);
            fflush(stdout);
            bb_make(game, best);
        }
        else
        {
            bb_make(game, 0);
        }
    }

    printf("\nGame Over!\n");
    bb_free(game);

    return 0;
}
