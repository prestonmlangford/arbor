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
    int p1_wins;
    int p2_wins;
    int visits;
    struct Node_t* sibling;
    struct Node_t* child;
} Node;

typedef struct Search_t
{
    Arbor_Search_Config cfg;

    Arbor_Game sim;
    Node* pool;
    int pool_count;
    int pool_limit;
} Search;

static int arbor_go(Search* search, Node* node);
static int arbor_leaf(Search* search, Node* node);

/*------------------------------------------------------------------------------
 * Private functions
 *----------------------------------------------------------------------------*/

static void arbor_expand(Search* search, Node* parent)
{
    Node** list = &(parent->child);
    int actions = arbor_actions(search->sim);
    int i = 0;

    for (i = 0; i < actions; i++)
    {
        if (search->pool_count < search->pool_limit)
        {
            Node* next = &(search->pool[search->pool_count]);

            search->pool_count += 1;
            next->action = i;
            *list = next;
            list = &(next->sibling);
        }
    }
}

static int arbor_branch(Search* search, Node* parent)
{
    Node* child = parent->child;
    Node* best = NULL;
    double best_uct = 0.0;
    double logN = log(parent->visits);

    if ((child == NULL) && (search->pool_count < search->pool_limit))
    {
        arbor_expand(search, parent);
        child = parent->child;
    }

    while (child)
    {
        double visits = (double) child->visits;
        double c = search->cfg.exploration;
        double exploration = sqrt(c*logN/visits);
        double wins = 0.0;
        double uct = 0.0;
        double exploitation = 0.0;

        if (child->visits == 0)
        {
            best = child;
            break;
        }
        else if (child->side == ARBOR_NONE)
        {
            if (child->result == parent->side)
            {
                wins = (double) visits;
            }
            else
            {
                wins = 0.0;
            }
        }
        else
        {
            switch (parent->side)
            {
            case ARBOR_P1:
                wins = child->p1_wins;
                break;

            case ARBOR_P2:
                wins = child->p2_wins;
                break;
            
            default:
                // this should never happen!
                assert(false);
                break;
            }
        }

        exploitation = wins / visits;

        uct = exploitation + exploration;

        if (best_uct < uct)
        {
            best_uct = uct;
            best = child;
        }

        child = child->sibling;
    }

    if (best)
    {
        arbor_make(search->sim, best->action);

        return arbor_go(search, best);
    }
    else
    {
        // ran out of memory
        return arbor_leaf(search, parent);
    }
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

static int arbor_go(Search* search, Node* node)
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
        result = arbor_branch(search, node);
    }
    else
    {
        // default evaluation policy
        result = arbor_leaf(search, node);
    }

    switch (result)
    {
    case ARBOR_P1:
        node->p1_wins += 1;
        break;

    case ARBOR_P2:
        node->p2_wins += 1;
        break;

    case ARBOR_DRAW:
        break;
    
    default:
        // should never happen!
        assert(false);
        break;
    }

    node->visits += 1;

    return result;
}

/*------------------------------------------------------------------------------
 * Public functions
 *----------------------------------------------------------------------------*/
Arbor_Search arbor_search_new(Arbor_Search_Config* cfg)
{
    Search* search = ARBOR_MALLOC(sizeof(Search));

    search->cfg = *cfg;
    search->pool = ARBOR_MALLOC(cfg->size);
    search->pool_count = 1;
    search->pool_limit = cfg->size / sizeof(Node);

    memset(search->pool, 0, cfg->size);

    return (Arbor_Search){search};
}

void arbor_search_delete(Arbor_Search search)
{
    Search* s = search.p;

    if (s)
    {
        ARBOR_FREE(s->pool);
        ARBOR_FREE(s);
    }
}

int arbor_search_best(Arbor_Search search)
{
    Search* s = search.p;
    Node* root = s->pool;
    Node* child = root->child;
    int best = child->action;
    double best_score = 0.0;

    while (child)
    {
        double visits = (double) child->visits;
        double score = 0.0;
        double wins = 0.0;

        if (child->side == ARBOR_NONE)
        {
            if (child->result == root->side)
            {
                wins = visits;
            }
            else
            {
                wins = 0.0;
            }
        }
        else
        {
            switch (root->side)
            {
            case ARBOR_P1:
                wins = (double) child->p1_wins;
                break;

            case ARBOR_P2:
                wins = (double) child->p2_wins;
                break;

            default:
                // should never happen!
                assert(false);
                break;
            }
        }

        score = wins / visits;

        if (best_score < score)
        {
            best_score = score;
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

    arbor_go(s, s->pool);

    arbor_delete(s->sim);
}
