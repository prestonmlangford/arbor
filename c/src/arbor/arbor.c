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
    struct Node_t* free_list;
    Arbor_Game game;
} Node;

typedef struct Search_t
{
    Arbor_Search_Config cfg;
    Arbor_Game_Interface ifc;

    Arbor_Game sim;
    Node* root;
    Node* free_list;
} Search;

static int arbor_go(Search* search, Node* node);

/*------------------------------------------------------------------------------
 * Private functions
 *----------------------------------------------------------------------------*/

static Node* arbor_new_node(Search* search, Arbor_Game game, int action)
{
    Node* node = ARBOR_MALLOC(sizeof(Node));

    node->free_list = search->free_list;
    search->free_list = node;

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

static int arbor_branch(Search* search, Node* parent)
{
    Node** list = &(parent->child);
    Node* best = *list;
    double logN = log(parent->visits);
    double best_uct = 0.0;
    int i = 0;

    for (i = 0; i < parent->actions; i++)
    {
        Node* child = *list;

        if (child == NULL)
        {
            search->ifc.make(search->sim, i);

            child = arbor_new_node(search, search->sim, i);

            *list = child;

            return arbor_go(search, child);
        }
        else if (child->side == ARBOR_NONE) // terminal condition
        {
            if (parent->side == child->result)
            {
                return child->result;
            }
        }
        else
        {
            double visits = (double) child->visits;
            double c = search->cfg.exploration;
            double exploration = sqrt(c*logN/visits);
            double uct = 0.0;
            double exploitation = 0.0;

            if (parent->side == child->side)
            {
                double wins = (double) child->wins;
                exploitation = 0.5 * wins / visits;
            }
            else
            {
                double losses = (double) child->losses;
                exploitation = 0.5 * losses / visits;
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
            int action = rand_bound(count);

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
    else if (node->visits > search->cfg.expansion)
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
        node->wins += 1;
        node->losses += 1;
    }
    else if (result == node->side)
    {
        node->wins += 2;
    }
    else
    {
        node->losses += 2;
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
    Search* search = ARBOR_MALLOC(sizeof(Search));

    search->cfg = *cfg;
    search->ifc = *ifc;
    search->root = arbor_new_node(search, search->cfg.init, 0);

    result.p = search;

    return result;
}

void arbor_search_delete(Arbor_Search search)
{
    Search* s = search.p;

    if (s)
    {
        Node* list = s->free_list;
        while (list)
        {
            Node* tmp = list->free_list;
            s->ifc.delete(list->game);
            ARBOR_FREE(list);
            list = tmp;
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
        double score = 0.0;

        if (child->result == root->side)
        {
            return child->action;
        }

        if (root->side == child->side)
        {
            double wins = (double) child->wins;
            score = 0.5 * wins/visits;
        }
        else
        {
            double losses = (double) child->losses;
            score = 0.5 * losses/visits;
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

    s->sim = s->ifc.copy(s->cfg.init);

    arbor_go(s, s->root);

    s->ifc.delete(s->sim);
}
