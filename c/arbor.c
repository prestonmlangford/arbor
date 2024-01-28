/*---------------------------------------------------------------------------
 * Copyright (C) 2024 by Preston Langford                                   *
 *                                                                          *
 *   This file is part of Arbor.                                            *
 *                                                                          *
 *   Arbor is free software: you can redistribute it and/or modify it         *
 *   under the terms of the GNU Lesser General Public License as published  *
 *   by the Free Software Foundation, either version 3 of the License, or   *
 *   (at your option) any later version.                                    *
 *                                                                          *
 *   Arbor is distributed in the hope that it will be useful,               *
 *   but WITHOUT ANY WARRANTY; without even the impied warranty of         *
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
#include <stdio.h>
#include <assert.h>
#include "arbor.h"
#include "random.h"

/*------------------------------------------------------------------------------
 * Private Types
 *----------------------------------------------------------------------------*/
enum
{
    ARBOR_TERMINAL,
    ARBOR_LEAF,
    ARBOR_BRANCH
};

typedef struct Node_t
{
    int side;
    int result;
    int action;
    int actions;
    int wins;
    int losses;
    int visits;
    struct Node_t* sibling;
    struct Node_t* child;
} Node;

typedef struct Search_t
{
    Arbor_Search_Config cfg;
    Arbor_Game_Interface ifc;

    Arbor_Game sim;
    Node* pool;
    size_t pool_count;
    size_t pool_size;
} Search;

static int arbor_go(Search* search, Node* node);

/*------------------------------------------------------------------------------
 * Private functions
 *----------------------------------------------------------------------------*/

static Node* arbor_pool_bump(Search* search)
{
    size_t top = search->pool_count;
    if (top < search->pool_size)
    {
        search->pool_count += 1;
        return &(search->pool[top]);
    }
    return NULL;
}

static Node* arbor_new_node(Search* search, Arbor_Game game, int action)
{
    Node* node = arbor_pool_bump(search);
    node->side = search->ifc.side(game);

    if (node->side == ARBOR_NONE)
    {
        node->result = search->ifc.eval(game);
    }
    else
    {
        node->result = ARBOR_NONE;
        node->actions = search->ifc.actions(game);
    }
    
    node->wins = 0;
    node->visits = 0;
    node->action = action;
    node->sibling = NULL;
    node->child = NULL;

    return node;
}

static int arbor_branch(Search* search, Node* node)
{
    Node** list = &(node->child);
    Node* best = *list;
    double logN = log(node->visits);
    double best_uct = 0.0;
    int i = 0;

    for (i = 0; i < node->actions; i++)
    {
        Node* child = *list;

        if (child == NULL)
        {
            search->ifc.make(search->sim, i);

            child = arbor_new_node(search, search->sim, i);

            *list = child;

            return arbor_go(search, child);
        }
        else if (child->result == node->side)
        {
            return child->result;
        }
        else
        {
            double visits = (double) child->visits;
            double c = search->cfg.exploration;
            double exploration = sqrt(c*logN/visits);
            double uct = 0.0;
            double exploitation = 0.0;

            if (node->side == child->side)
            {
                double wins = (double) child->wins;
                exploitation = wins / visits;
            }
            else
            {
                double losses = (double) child->losses;
                exploitation = losses / visits;
            }

            uct = exploitation + exploration;

            if (best_uct < uct)
            {
                best_uct = uct;
                best = child;
            }
        }

        list  = &(child->sibling);
    }

    search->ifc.make(search->sim, best->action);

    return arbor_go(search, best);
}

static int arbor_leaf(Search* search, Node* node)
{
    if (search->cfg.eval_policy == ARBOR_EVAL_ROLLOUT)
    {
        while (search->ifc.side(search->sim) != ARBOR_NONE)
        {
            int count = search->ifc.actions(search->sim);
            int action = rand_range(0, count);

            search->ifc.make(search->sim, action);
        }
    }

    return search->ifc.eval(search->sim);
}

static int arbor_go(Search* search, Node* node)
{
    int result = ARBOR_NONE;

    if (node->side == ARBOR_NONE)
    {
        result = node->result;
    }
    else if (node->visits >= search->cfg.expansion)
    {
        result = arbor_branch(search, node);
    }
    else
    {
        // default evaluation policy
        result = arbor_leaf(search, node);
    }

    if (result == ARBOR_DRAW)
    {
        /* do nothing */
    }
    else if (result == node->side)
    {
        node->wins += 1;
    }
    else
    {
        node->losses += 1;
    }

    node->visits += 1;

    return result;
}

/*------------------------------------------------------------------------------
 * Public functions
 *----------------------------------------------------------------------------*/
Arbor_Search arbor_search_new(Arbor_Search_Config* cfg,
                              Arbor_Game_Interface* ifc)
{
    Arbor_Search result = {};
    Search* search = malloc(sizeof(Search));
    Node* root = malloc(cfg->size);

    search->cfg = *cfg;
    search->ifc = *ifc;
    search->pool = root;
    search->pool_size = cfg->size / sizeof(Node);
    search->pool_count = 0;

    (void) arbor_new_node(search, search->cfg.init, 0);

    result.p = search;

    return result;
}

void arbor_search_free(Arbor_Search search)
{
    Search* s = search.p;

    if (s)
    {
        free(s->pool);
        free(s);
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

        if (child->result == root->side)
        {
            return child->action;
        }

        if (root->side == child->side)
        {
            double wins = (double) child->wins;
            score = wins/visits;
        }
        else
        {
            double losses = (double) child->losses;
            score = losses/visits;
        }

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
    Node* root = s->pool;

    s->sim = s->ifc.copy(s->cfg.init);

    arbor_go(s, root);

    s->ifc.free(s->sim);
}
