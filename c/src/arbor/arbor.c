/*---------------------------------------------------------------------------
 * Copyright (C) 2024 by Preston Langford                                   *
 *                                                                          *
 *   This file is part of Arbor.                                            *
 *                                                                          *
 *   Arbor is free software: you can redistribute it and/or modify it       *
 *   under the terms of the GNU Lesser General Public License as published  *
 *   by the Free Software Foundation, either version 3 of the License, or   *
 *   (at your option) any later version.                                    *
 *                                                                          *
 *   Arbor is distributed in the hope that it will be useful,               *
 *   but WITHOUT ANY WARRANTY; without even the impied warranty of          *
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
 *   GNU Lesser General Public License for more details.                    *
 *                                                                          *
 *   You should have received a copy of the GNU Lesser General Public       *
 *   License along with Arbor.  If not, see <http://www.gnu.org/licenses/>. *
 ---------------------------------------------------------------------------*/

/*------------------------------------------------------------------------------
 * @file arbor.c
 *
 * @author Preston Langford
 * @date 19 Jan 2024
 * 
 * @brief Impementation of arbor.
 *
 *----------------------------------------------------------------------------*/
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>
#include "arbor.h"
#include "random.h"

/*------------------------------------------------------------------------------
 * Private Types
 *----------------------------------------------------------------------------*/

typedef struct Node_t
{
    int side;
    int result;
    int action;
    int value;
    int visits;
    struct Node_t* sibling;
    struct Node_t* child;
    struct Node_t* free;
} Node;

#ifdef ARBOR_METRICS
typedef struct Metrics_t
{   
    int size_node;
    int num_nodes;
    int num_branches;
    int max_depth;
} Metrics;
#endif //ARBOR_METRICS

typedef struct Search_t
{
    Arbor_Search_Config cfg;
    Arbor_Game sim;
    Node* root;
    Node* free;

#ifdef ARBOR_METRICS
    Metrics metrics;
#endif //ARBOR_METRICS

} Search;

static int arbor_go(Search* search, Node* node, int depth);
static int arbor_leaf(Search* search, Node* node);

/*------------------------------------------------------------------------------
 * Private functions
 *----------------------------------------------------------------------------*/

static Node* arbor_node(Search* search)
{
    Node* node = ARBOR_MALLOC(sizeof(Node));
    
    (void) memset(node, 0, sizeof(Node));

    node->free = search->free;
    search->free = node;

#ifdef ARBOR_METRICS
    search->metrics.num_nodes++;
#endif //ARBOR_METRICS

    return node;
}

static void arbor_expand(Search* search, Node* parent)
{
    Node** list = &(parent->child);
    int actions = arbor_actions(search->sim);
    int i = 0;

#ifdef ARBOR_METRICS
    search->metrics.num_branches++;
#endif //ARBOR_METRICS

    for (i = 0; i < actions; i++)
    {
        Node* next = arbor_node(search);

        next->action = i;
        *list = next;
        list = &(next->sibling);
    }
}

static int arbor_branch(Search* search, Node* parent, int depth)
{
    Node* child = parent->child;
    Node* best = NULL;
    double best_uct = -1.0;
    double logN = log(parent->visits);

    if (child == NULL)
    {
        arbor_expand(search, parent);
        child = parent->child;
    }

    while (child)
    {
        double visits = (double) child->visits;
        double c = search->cfg.exploration;
        double exploration = sqrt(c*logN/visits);
        double uct = 0.0;
        double exploitation = 0.0;

        if (child->visits == 0)
        {
            best = child;
            break;
        }
        else if (child->result == ARBOR_P1)
        {
            exploitation = 1.0;
        }
        else if (child->result == ARBOR_P2)
        {
            exploitation = 0.0;
        }
        else if (child->result == ARBOR_DRAW)
        {
            exploitation = 0.5;
        }
        else
        {
            double value = 0.5 * ((double) child->value);
            exploitation = value / visits;
        }

        if (parent->side == ARBOR_P2)
        {
            exploitation = 1.0 - exploitation;
        }

        uct = exploitation + exploration;

        if (best_uct < uct)
        {
            best_uct = uct;
            best = child;
        }

        child = child->sibling;
    }

    arbor_make(search->sim, best->action);

    return arbor_go(search, best, depth + 1);
}

static int arbor_leaf(Search* search, Node* node)
{
    if (search->cfg.eval_policy == ARBOR_EVAL_ROLLOUT)
    {
        while (arbor_side(search->sim) != ARBOR_NONE)
        {
            int count = arbor_actions(search->sim);
            int action = rand_bound(count);

            arbor_make(search->sim, action);
        }
    }

    return arbor_eval(search->sim);
}

static int arbor_go(Search* search, Node* node, int depth)
{
    int result = ARBOR_NONE;

    if (node->visits == 0)
    {
        node->side = arbor_side(search->sim);

        if (node->side == ARBOR_NONE)
        {
            node->result = arbor_eval(search->sim);
        }
        else
        {
            node->result = ARBOR_NONE;
        }
    }

    if (node->side == ARBOR_NONE)
    {
        result = node->result;
    }
    else if (node->visits > search->cfg.expansion)
    {
        result = arbor_branch(search, node, depth);
    }
    else
    {
        // default evaluation policy
        result = arbor_leaf(search, node);
    }

    switch (result)
    {
    case ARBOR_P1:
        node->value += 2;
        break;

    case ARBOR_P2:
        break;

    case ARBOR_DRAW:
        node->value += 1;
        break;
    
    default:
        // should never happen!
        assert(false);
        break;
    }

#ifdef ARBOR_METRICS
    if (search->metrics.max_depth < depth)
    {
        search->metrics.max_depth = depth;
    }
#endif //ARBOR_METRICS

    node->visits += 1;

    return result;
}

/*------------------------------------------------------------------------------
 * Public functions
 *----------------------------------------------------------------------------*/
Arbor_Search arbor_search_new(Arbor_Search_Config* cfg)
{
    Search* search = ARBOR_MALLOC(sizeof(Search));

    (void) memset(search, 0, sizeof(Search));

    search->cfg = *cfg;
    search->free = NULL;
    search->root = arbor_node(search);

#ifdef ARBOR_METRICS
    search->metrics.size_node = sizeof(Node);
#endif //ARBOR_METRICS

    return (Arbor_Search){search};
}

void arbor_search_delete(Arbor_Search search)
{
    Search* s = search.p;

    if (s)
    {
        Node* f = s->free;

        while (f)
        {
            Node* tmp = f->free;
            ARBOR_FREE(f);
            f = tmp;
        }

        ARBOR_FREE(s);
    }
}

int arbor_search_best(Arbor_Search search)
{
    Search* s = search.p;
    Node* root = s->root;
    Node* child = root->child;
    int best = child->action;
    double best_score = 0.0;

    while (child)
    {
        double visits = (double) child->visits;
        double exploitation = 0.0;

        if (child->result == ARBOR_P1)
        {
            exploitation = 1.0;
        }
        else if (child->result == ARBOR_P2)
        {
            exploitation = 0.0;
        }
        else if (child->result == ARBOR_DRAW)
        {
            exploitation = 0.5;
        }
        else
        {
            double value = 0.5 * ((double) child->value);
            exploitation = value / visits;
        }

        if (root->side == ARBOR_P2)
        {
            exploitation = 1.0 - exploitation;
        }

        if (best_score < exploitation)
        {
            best_score = exploitation;
            best = child->action;
        }

        child = child->sibling;
    }

    return best;
}

void arbor_search_ponder(Arbor_Search search)
{
    Search* s = search.p;

    s->sim = arbor_copy(s->cfg.init);

    arbor_go(s, s->root, 0);

    arbor_delete(s->sim);
}

#ifdef ARBOR_METRICS
void arbor_show_metrics(Arbor_Search search)
{
    Search* s = search.p;
    double value = 0.5 * ((double) s->root->value);
    double visits = (double) s->root->visits;
    double p1_value = 100.0 * value / visits;
    double p2_value = 100.0 - p1_value;

    printf("----------------------------------\n");
    printf("size_node    = %d\n", s->metrics.size_node);
    printf("num_nodes    = %d\n", s->metrics.num_nodes);
    printf("num_branches = %d\n", s->metrics.num_branches);
    printf("max_depth    = %d\n", s->metrics.max_depth);
    printf("iterations   = %d\n", s->root->visits);
    printf("p1           = %2.1f\n", p1_value);
    printf("p2           = %2.1f\n", p2_value);
    printf("----------------------------------\n");
}
#endif //ARBOR_METRICS
