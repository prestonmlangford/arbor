/*---------------------------------------------------------------------------
 * Copyright (C) 2024 by Preston Langford                                   *
 *                                                                          *
 *   This file is part of Arbor.                                            *
 *                                                                          *
 *   Box is free software: you can redistribute it and/or modify it         *
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
#include "math.h"
#include "arbor.h"

/*------------------------------------------------------------------------------
 * Private Types
 *----------------------------------------------------------------------------*/
typedef enum NodeType_t
{
    UNKOWN,
    TERMINAL,
    LEAF,
    BRANCH,
    TRANSPOSE
} NodeType;

typedef struct Node_t
{
    NodeType type;
    Arbor_Game_Result result;
    int action;
    int player;
    int wins;
    int visits;
    Node* sibling;
    Node* child;
} Node;

typedef struct Search_t
{
    Arbor_Game game;
    Node* pool;
    size_t pool_count;
    size_t pool_size;
    double exploration;
} Search;

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

static Node* arbor_choose(Search* search, Node* node)
{
    Node* next = node->child;
    Node* best = next;
    double best_uct = 0.0;
    double logN = log(node->visits);
    
    while (next)
    {
        if (next->type == TERMINAL)
        {
            if (next->result == ARBOR_GAME_RESULT_WIN)
            {
                return next;
            }            
        }

        next = next->sibling;
    }
    
    next = node->child;
    while (next)
    {
        if (next->type == UNKOWN)
        {
            return next;
        }

        next = next->sibling;
    }
    
    next = node->child;
    while (next)
    {
        if ((next->type == LEAF) || (next->type == BRANCH))
        {
            double w = (double) next->wins;
            double n = (double) next->visits;
            double c = search->exploration;
            double exploitation = w/n;
            double exploration = c*sqrt(logN/n);
            double uct = exploitation + exploration;

            if (best_uct < uct)
            {
                best_uct = uct;
                best = next;
            }
        }

        next = next->sibling;
    }
    
    return best;
}

static int arbor_eval(Search* search, Node* node)
{
    switch (node->type)
    {            
        case BRANCH:
            break;

        case LEAF:
            break;

        case TERMINAL:
            break;

        case UNKOWN:
            break;

        case TRANSPOSE:
            break;
        
        default:
            break;
    }
}

/*------------------------------------------------------------------------------
 * Public functions
 *----------------------------------------------------------------------------*/
Arbor_Search arbor_search_new(Arbor_Game game, size_t size)
{
    Arbor_Search result = {};
    
    if (size > sizeof(Node))
    {
        Search* search = malloc(sizeof(Search));
        Node* pool = malloc(size);
        
        search->game = game;
        search->pool = pool;
        search->pool_size = size / sizeof(Node);
        search->pool_count = 0;
    }

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
    return 0;
}

void arbor_search_ponder(Arbor_Search search)
{
    
}
