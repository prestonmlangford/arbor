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

/*------------------------------------------------------------------------------
 * User types
 *----------------------------------------------------------------------------*/
OPAQUE_TYPE_DECL(Arbor_Game);
typedef enum Arbor_Game_Result_t
{
    ARBOR_GAME_RESULT_NONE,
    ARBOR_GAME_RESULT_WIN,
    ARBOR_GAME_RESULT_LOSE,
    ARBOR_GAME_RESULT_DRAW
} Arbor_Game_Result;

/*------------------------------------------------------------------------------
 * Lib types
 *----------------------------------------------------------------------------*/
OPAQUE_TYPE_DECL(Arbor_Search);

/*------------------------------------------------------------------------------
 * User functions
 *----------------------------------------------------------------------------*/

/*------------------------------------------------------------------------------
 * @fn arbor_game_actions
 *
 * @brief Indicate the number of actions for the given game state. The game 
 *        state implementation must always generate the same number of moves in
 *        the same order. 
 * 
 * @param [in]  game    The game state to iterate.
 * 
 * @return Number of actions possible for game state.
 *         Zero when there are no actions.
 *----------------------------------------------------------------------------*/
int arbor_game_actions(Arbor_Game game);

/*------------------------------------------------------------------------------
 * @fn arbor_game_make
 *
 * @brief  Advance the game state with the given action index. All game state
 *         actions must be generated in a consistent order each time for the
 *         index to work correctly.
 * 
 * @param [in]  game    The game state.
 * @param [in]  action  index of action to make.
 * 
 * @return true when the action successfully advances the game state.
 *         false otherwise.
 *----------------------------------------------------------------------------*/
bool arbor_game_make(Arbor_Game game, int action);

/*------------------------------------------------------------------------------
 * @fn arbor_game_side
 *
 * @brief Indicate side to play.
 * 
 * @param [in]  game  The game state.
 * 
 * @return true when it is the first players turn.
 *         false otherwise.
 *----------------------------------------------------------------------------*/
bool arbor_game_side(Arbor_Game game);

/*------------------------------------------------------------------------------
 * @fn arbor_game_over
 *
 * @brief Indicate the state of the game.
 * 
 * @param [in]  game  The game state.
 * 
 * @return One of the following Arbor_Game_Result variants:
 *         ARBOR_GAME_RESULT_NONE game is still in play.
 *         ARBOR_GAME_RESULT_WIN  current player has won.
 *         ARBOR_GAME_RESULT_LOSE current player has lost.
 *         ARBOR_GAME_RESULT_DRAW neither play has won.
 *----------------------------------------------------------------------------*/
Arbor_Game_Result arbor_game_over(Arbor_Game game);

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
Arbor_Search arbor_search_new(Arbor_Game game, size_t size);

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
