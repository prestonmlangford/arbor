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

#ifndef ARBOR_FREE
#define ARBOR_FREE free
#endif //ARBOR_FREE

#ifndef ARBOR_MALLOC
#define ARBOR_MALLOC malloc
#endif //ARBOR_MALLOC

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

/*------------------------------------------------------------------------------
 * Required user implemented functions
 *----------------------------------------------------------------------------*/

/*------------------------------------------------------------------------------
 * @fn Arbor_Copy
 *
 * @brief Allocate resources and deep copy the given game state. User can expect
 *        Arbor to copy and free the game state for each call to ponder. It may 
 *        copy more if multi-threading is used.
 *
 * @param [in]  game  The game state.
 * 
 * @return A copy of an existing Arbor_Game.
 *----------------------------------------------------------------------------*/
Arbor_Game arbor_copy(Arbor_Game game);

/*------------------------------------------------------------------------------
 * @fn Arbor_Delete
 *
 * @brief Deallocate any user resources used to copy the initial game state.
 *
 * @param [in]  game  The game state.
 * 
 * @return None.
 *----------------------------------------------------------------------------*/
void arbor_delete(Arbor_Game game);

/*------------------------------------------------------------------------------
 * @fn Arbor_Make
 *
 * @brief Perform the action and advance the game state. The user game must
 *        enumerate all actions in the same order.
 *
 * @param [in]  game  The game state.
 * 
 * @return None.
 *----------------------------------------------------------------------------*/
void arbor_make(Arbor_Game game, int action);

/*------------------------------------------------------------------------------
 * @fn Arbor_Actions
 *
 * @brief Indicate the number of actions available to the current player. The 
 *        actions must be enumerated in a deterministic way by the game.
 *
 * @param [in]  game  The game state.
 * 
 * @return The number of actions available to the current player. Must be > 0
 *         if the side to play is != ARBOR_NONE.
 *----------------------------------------------------------------------------*/
int arbor_actions(Arbor_Game game);

/*------------------------------------------------------------------------------
 * @fn Arbor_Side
 *
 * @brief Indicate side to play for the current game state.
 *
 * @param [in]  game  The game state.
 * 
 * @return One of the following:
 *         ARBOR_NONE The game is in a terminal state.
 *         ARBOR_P1   1st player won.
 *         ARBOR_P2   2nd player won.
 *----------------------------------------------------------------------------*/
int arbor_side(Arbor_Game game);

/*------------------------------------------------------------------------------
 * @fn Arbor_Eval
 *
 * @brief Indicate result of game if in a terminal state, or pick a winner based
 * on the probability one side might win vs. the other.
 *
 * @param [in]  game  The game state.
 * 
 * @return One of the following:
 *         ARBOR_P1   1st player won.
 *         ARBOR_P2   2nd player won.
 *         ARBOR_DRAW neither player won.
 *----------------------------------------------------------------------------*/
int arbor_eval(Arbor_Game game);

/*------------------------------------------------------------------------------
 * Optional user implemented functions
 *----------------------------------------------------------------------------*/

/*------------------------------------------------------------------------------
 * @fn Arbor_Copy
 *
 * @brief Start a new game.
 *
 * @param [in]  game  The game state.
 * 
 * @return A new Arbor_Game.
 *----------------------------------------------------------------------------*/
Arbor_Game arbor_new(void);

/*------------------------------------------------------------------------------
 * @fn Arbor_Show
 *
 * @brief emit a representation of the game to stdout.
 *
 * @param [in]  game  The game state.
 * 
 * @return None.
 *----------------------------------------------------------------------------*/
void arbor_show(Arbor_Game game);

/*------------------------------------------------------------------------------
 * @fn Arbor_Vector
 *
 * @brief emit a vector of features for training.
 *
 * @param [in]  game  The game state.
 * 
 * @return None.
 *----------------------------------------------------------------------------*/
void arbor_vector(Arbor_Game game);

/*------------------------------------------------------------------------------
 * @fn Arbor_Prob
 *
 * @brief emit win probability from heuristic.
 *
 * @param [in]  game  The game state.
 * 
 * @return None.
 *----------------------------------------------------------------------------*/
void arbor_prob(Arbor_Game game);

/*------------------------------------------------------------------------------
 * Lib functions
 *----------------------------------------------------------------------------*/

typedef struct Arbor_Search_Config_t
{
    Arbor_Game init;
    size_t size;
    int expansion;
    double exploration;
    int eval_policy;
} Arbor_Search_Config;

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
Arbor_Search arbor_search_new(Arbor_Search_Config* cfg);

/*------------------------------------------------------------------------------
 * @fn arbor_search_delete
 *
 * @brief Deallocates resources used by the search.
 * 
 * @param [in] search  Handle for the search to free.
 *----------------------------------------------------------------------------*/
void arbor_search_delete(Arbor_Search search);

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
