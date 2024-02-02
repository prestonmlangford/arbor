#include <time.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>
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

int reversi_ai(Arbor_Game game, int ms)
{
    Arbor_Game_Interface ifc = {
        .actions = reversi_actions,
        .copy = reversi_copy,
        .delete = reversi_delete,
        .make = reversi_make,
        .eval = reversi_eval,
        .side = reversi_side
    };
    Arbor_Search_Config cfg = {
        .expansion = 10,
        .exploration = 2.0,
        .init = game,
        .eval_policy = ARBOR_EVAL_ROLLOUT
    };

    Arbor_Search search = arbor_search_new(&cfg, &ifc);
    clock_t now, future;
    int action = 0;

    now = clock();
    future = now + ((ms * CLOCKS_PER_SEC) / 1000);

    while (now < future)
    {
        arbor_search_ponder(search);
        now = clock();
    }

    action = arbor_search_best(search);

    arbor_search_delete(search);

    return action;
}

int reversi_cli(Arbor_Game game)
{
    int xy;

    printf(">> ");
    scanf("%o",&xy);
    printf("\n");

    return reversi_convert_xy(game, xy);
}

void reversi_manual(void)
{
    Arbor_Game game = reversi_new();

    for (;;)
    {
        int action, side;

        reversi_show(game);
        side = reversi_side(game);

        if (side == ARBOR_NONE)
        {
            break;
        }
        else
        {
            action = reversi_cli(game);
        }

        reversi_make(game, action);
    }
}

void reversi_match(void)
{
    Arbor_Game game = reversi_new();

    for (;;)
    {
        int action, side;

        reversi_show(game);
        side = reversi_side(game);

        if (side == ARBOR_P1)
        {
            action = reversi_cli(game);
        }
        else if (side == ARBOR_P2)
        {
            action = reversi_ai(game, 500);
        }
        else
        {
            break;
        }

        reversi_make(game, action);

    }
}

int main (int argc, char* argv[])
{
    // profile(bad_battleship);
    // profile(dice);
    // profile(rps);

    int i, xy, action, side;
    Arbor_Game game = reversi_new();

    // reversi_match();
    // reversi_manual();

    for (i = 1; i < argc; i++)
    {
        const char* arg = argv[i];
        // printf("%s\n",arg);

        if (strcmp(arg, "show") == 0)
        {
            reversi_show(game);
            return 0;
        }
        else if (strcmp(arg, "pass") == 0)
        {
            action = 0;
        }
        else
        {
            sscanf(arg, "%o",&xy);
            action = reversi_convert_xy(game, xy);
        }

        reversi_make(game, action);
        side = reversi_side(game);

        if (side == ARBOR_NONE)
        {
            int result = reversi_eval(game);

            if (result == ARBOR_P1)
            {
                printf("white\n");
            }
            else if (result == ARBOR_P2)
            {
                printf("black\n");
            }
            else
            {
                printf("draw\n");
            }

            return 0;
        }
    }

    action = reversi_ai(game, 1000);
    xy = reversi_convert_action(game, action);

    if (xy >= 0)
    {
        printf("%02o\n", xy);
    }
    else
    {
        printf("pass\n");
    }

    return 0;
}
