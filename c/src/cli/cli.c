#include <time.h>
#include <stdio.h>
#include <string.h>
#include "arbor.h"
#include "random.h"
#include "profile.h"

#define KB 1024
#define MB (KB * KB)
#define NS_PER_SEC UINT64_C(1000000000)
#define NS_PER_MS UINT64_C(1000000)

static int eval_policy = ARBOR_EVAL_ROLLOUT;

static int rollout(Arbor_Game game, Arbor_Game_Interface* ifc)
{
    Arbor_Game sim = ifc->copy(game);
    int result = ARBOR_NONE;

    while (ifc->side(sim) != ARBOR_NONE)
    {
        int count = ifc->actions(sim);
        int action = rand_bound(count);

        ifc->make(sim, action);
    }

    result = ifc->eval(sim);
    ifc->delete(sim);

    return result;
}

static uint64_t time_ns(void)
{
	struct timespec ts;
    uint64_t ns = 0;

    clock_gettime(CLOCK_MONOTONIC, &ts);

    ns = ts.tv_sec;
    ns *= NS_PER_SEC;
    ns += ts.tv_nsec;

    return ns;
}

static int timed_ai(Arbor_Game game, Arbor_Game_Interface* ifc, int ms)
{
    Arbor_Search_Config cfg = {
        .expansion = 0,
        .exploration = 2.0,
        .init = game,
        .eval_policy = eval_policy,
        .size = 500 * MB
    };

    Arbor_Search search = arbor_search_new(&cfg, ifc);
	uint64_t now, future;
    int count = 0;
    int action = 0;

    now = time_ns();
    future = now + (ms * NS_PER_MS);

    while (now < future)
    {
        arbor_search_ponder(search);
        now = time_ns();
        count++;
    }

    fprintf(stderr, "c iterations %d\n", count);
    action = arbor_search_best(search);

    arbor_search_delete(search);

    return action;
}

static int bounded_ai(Arbor_Game game, Arbor_Game_Interface* ifc, int iter)
{
    Arbor_Search_Config cfg = {
        .expansion = 0,
        .exploration = 2.0,
        .init = game,
        .eval_policy = ARBOR_EVAL_ROLLOUT
    };

    Arbor_Search search = arbor_search_new(&cfg, ifc);
    int count = 0;
    int action = 0;

    while (count < iter)
    {
        arbor_search_ponder(search);
        count++;
    }

    // fprintf(stderr, "c iterations %d\n", count);
    action = arbor_search_best(search);

    arbor_search_delete(search);

    return action;
}

int cli(Arbor_Game game, Arbor_Game_Interface* ifc, int argc, char* argv[])
{
    int i, ms, iter, result, action, side;

    for (i = 1; i < argc; i++)
    {
        const char* arg = argv[i];

        side = ifc->side(game);

        if (strcmp(arg, "show") == 0)
        {
            ifc->show(game);
        }
        else if (strcmp(arg, "vector") == 0)
        {
            ifc->vector(game);
        }
        else if (strcmp(arg, "prob") == 0)
        {
            ifc->prob(game);
        }
        else if (strcmp(arg, "policy:rollout") == 0)
        {
            eval_policy = ARBOR_EVAL_ROLLOUT;
        }
        else if (strcmp(arg, "policy:custom") == 0)
        {
            eval_policy = ARBOR_EVAL_CUSTOM;
        }
        else if (sscanf(arg,"mcts:time:%d",&ms) == 1)
        {
            if (side == ARBOR_NONE)
            {
                fprintf(stderr, "error - game over\n");
                return -1;
            }
            else
            {
                action = timed_ai(game, ifc, ms);
                printf("%d\n", action);
            }
        }
        else if (sscanf(arg,"mcts:iter:%d",&iter) == 1)
        {
            if (side == ARBOR_NONE)
            {
                fprintf(stderr, "error - game over\n");
                return -1;
            }
            else
            {
                action = bounded_ai(game, ifc, iter);
                printf("%d\n", action);
            }
        }
        else if (sscanf(arg,"rollout:%d",&iter) == 1)
        {
            if (side == ARBOR_NONE)
            {
                fprintf(stderr, "error - game over\n");
                return -1;
            }
            else
            {
                int p1 = 0;
                int p2 = 0;

                rand_seed_realtime();

                while (iter > 0)
                {
                    result = rollout(game, ifc);
                    if (result == ARBOR_P1)
                    {
                        p1++;
                    }
                    else if (result == ARBOR_P2)
                    {
                        p2++;
                    }
                    iter--;
                }

                printf("%d,%d\n", p1, p2);
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
            else
            {
                printf("none\n");
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
