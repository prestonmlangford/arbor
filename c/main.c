#include <time.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <errno.h>
#include "arbor.h"
#include "random.h"
#include "profile.h"
#include "bad_battleship.h"
#include "dice.h"
#include "rps.h"
#include "reversi.h"

void bad_battleship(void)
{

    Arbor_Game_Interface ifc = {
        .actions = bb_actions,
        .copy = bb_copy,
        .delete = bb_delete,
        .make = bb_make,
        .eval = bb_eval,
        .side = bb_side
    };

    Arbor_Game game = bb_new();

    rand_seed_realtime();

    printf("Arbor - Bad Battleship\n");

    while (bb_side(game) != ARBOR_NONE)
    {
        if (bb_side(game) == ARBOR_P1)
        {
            Arbor_Search_Config cfg = {
                .expansion = 10,
                .exploration = 2.0,
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
            arbor_search_delete(search);

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
    bb_delete(game);
}

void dice(void)
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
}
void rps(void)
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
    dice_delete(game);
}

int mcts(Arbor_Game game, Arbor_Game_Interface* ifc, int ms)
{
    Arbor_Search_Config cfg = {
        .expansion = 10,
        .exploration = 2.0,
        .init = game,
        .eval_policy = ARBOR_EVAL_ROLLOUT
    };

    Arbor_Search search = arbor_search_new(&cfg, ifc);
    clock_t now, future;
    int count = 0;
    int action = 0;

    now = clock();
    future = now + ((ms * CLOCKS_PER_SEC) / 1000);

    while (now < future)
    {
        arbor_search_ponder(search);
        now = clock();
        count++;
    }

    fprintf(stderr, "c iterations %d\n", count);
    action = arbor_search_best(search);

    arbor_search_delete(search);

    return action;
}

int cli(Arbor_Game game, Arbor_Game_Interface* ifc, int argc, char* argv[])
{
    int i, ms, result, action, side;

    for (i = 1; i < argc; i++)
    {
        const char* arg = argv[i];

        side = ifc->side(game);

        if (strcmp(arg, "show") == 0)
        {
            ifc->show(game);
        }
        else if (sscanf(arg,"mcts:%d",&ms) == 1)
        {
            if (side == ARBOR_NONE)
            {
                fprintf(stderr, "error - game over\n");
                return -1;
            }
            else
            {
                action = mcts(game, ifc, ms);
                printf("%d\n", action);
            }
        }
        else if (strcmp(arg, "side") == 0)
        {
            if (side == ARBOR_P1)
            {
                printf("p1\n");
            }
            else if (side == ARBOR_P2)
            {
                printf("p2\n");
            }
            else
            {
                printf("none\n");
            }
        }
        else if (strcmp(arg, "actions") == 0)
        {
            printf("%d\n", ifc->actions(game));
        }
        else if (strcmp(arg, "result") == 0)
        {
            if (side == ARBOR_NONE)
            {
                printf("none\n");
            }
            else
            {
                result = ifc->eval(game);

                if (result == ARBOR_P1)
                {
                    printf("p1\n");
                }
                else if (result == ARBOR_P2)
                {
                    printf("p2\n");
                }
                else
                {
                    printf("draw\n");
                }
            }
        }
        else if (sscanf(arg, "%d", &action) == 1)
        {
            if (side == ARBOR_NONE)
            {
                fprintf(stderr, "error - game over\n");
                return -1;
            }
            else
            {
                ifc->make(game, action);
            }
        }
        else
        {
            fprintf(stderr, "error - arg %d -> %s\n", i, arg);
            return -1;
        }
    }

    return 0;
}

int reversi_cli(int argc, char* argv[])
{
    Arbor_Game_Interface ifc = {
        .actions = reversi_actions,
        .copy = reversi_copy,
        .delete = reversi_delete,
        .make = reversi_make,
        .eval = reversi_eval,
        .side = reversi_side,
        .show = reversi_show
    };
    Arbor_Game game = reversi_new();
    int result = cli(game, &ifc, argc, argv);

    reversi_delete(game);

    return result;
}

int main (int argc, char* argv[])
{
    // profile(bad_battleship);
    // profile(dice);
    // profile(rps);
    // reversi_match();
    // reversi_manual();
    // return 0;
    return reversi_cli(argc, argv);
}
