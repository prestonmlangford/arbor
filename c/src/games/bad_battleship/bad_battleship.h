
#ifndef BAD_BATTLESHIP_H
#define BAD_BATTLESHIP_H

#include "arbor.h"

Arbor_Game bb_copy(Arbor_Game game);
void bb_delete(Arbor_Game game);
void bb_make(Arbor_Game game, int action);
int bb_actions(Arbor_Game game);
int bb_side(Arbor_Game game);
int bb_eval(Arbor_Game game);
Arbor_Game bb_new(void);

#endif // BAD_BATTLESHIP_H
