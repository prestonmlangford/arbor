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
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of         *
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
 *   GNU Lesser General Public License for more details.                    *
 *                                                                          *
 *   You should have received a copy of the GNU Lesser General Public       *
 *   License along with Arbor.  If not, see <http://www.gnu.org/licenses/>. *
 ---------------------------------------------------------------------------*/

/*------------------------------------------------------------------------------
 * @file arbor.h
 *
 * @author Preston Langford
 * @date 19 Jan 2024
 * 
 * @brief Types and function prototypes for the arbor interface.
 *
 * The user implements the following arbor_game_* functions for arbor to search the
 * game state generically.
 *----------------------------------------------------------------------------*/

#ifndef ARBOR_H
#define ARBOR_H

#include <stdlib.h>
#include <stdbool.h>

#define OPAQUE_TYPE_DECL(name) typedef struct name##_t {void* p;} name;

OPAQUE_TYPE_DECL(Arbor_Search);
OPAQUE_TYPE_DECL(Arbor_Game);

enum
{
    ARBOR_NONE,
    ARBOR_P1,
    ARBOR_P2,
    ARBOR_DRAW,
    ARBOR_EVAL_ROLLOUT,
    ARBOR_EVAL_CUSTOM,
};

typedef struct Arbor_Game_Interface_t
{
    Arbor_Game (*copy)(Arbor_Game game);
    void (*free)(Arbor_Game game);
    void (*make)(Arbor_Game game, int action);
    int (*actions)(Arbor_Game game);
    int (*side)(Arbor_Game game);
    int (*eval)(Arbor_Game game);
} Arbor_Game_Interface;

typedef struct Arbor_Search_Config_t
{
    Arbor_Game init;
    size_t size;
    int expansion;
    double exploration;
    int eval_policy;
} Arbor_Search_Config;

/*------------------------------------------------------------------------------
 * @fn arbor_game_result
 *
 * @brief Indicate result of game.
 * 
 * @param [in]  game  The game state.
 * 
 * @return One of the following:
 *         ARBOR_NONE game is in play.
 *         ARBOR_P1   1st player won.
 *         ARBOR_P2   2nd player won.
 *         ARBOR_DRAW neither player won.
 *----------------------------------------------------------------------------*/
int arbor_game_result(Arbor_Game game);

/*------------------------------------------------------------------------------
 * Lib functions
 *----------------------------------------------------------------------------*/

/*------------------------------------------------------------------------------
 * @fn arbor_search_new
 *
 * @brief Allocate and return a new Arbor_Search object.
 * 
 * @param [in] game  The game state to search.
 * @param [in] size  Limit the size used by the search.
 * 
 * @return A new Arbor_Search search object.
 *----------------------------------------------------------------------------*/
Arbor_Search arbor_search_new(Arbor_Search_Config* cfg,
                              Arbor_Game_Interface* ifc);

/*------------------------------------------------------------------------------
 * @fn arbor_search_free
 *
 * @brief Deallocates resources used by the search.
 * 
 * @param [in] search  Handle for the search to free.
 *----------------------------------------------------------------------------*/
void arbor_search_free(Arbor_Search search);

/*------------------------------------------------------------------------------
 * @fn arbor_search_best
 *
 * @brief Picks the best available action in the current game state for the
 *        time spent pondering.
 * 
 * @param [in]  search  Handle for the search.
 * 
 * @return int >= 0 corresponding to the best action.
 *         int <  0 otherwise.
 *----------------------------------------------------------------------------*/
int arbor_search_best(Arbor_Search search);

/*------------------------------------------------------------------------------
 * @fn arbor_search_ponder
 * 
 * @brief Completes one iteration of MCTS. Keep calling this function as long as
 * time allows to improve the search accuracy.
 * 
 * @param [in] search  Handle for the active search.
 *----------------------------------------------------------------------------*/
void arbor_search_ponder(Arbor_Search search);

#endif // ARBOR_H
