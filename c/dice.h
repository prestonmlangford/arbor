
#ifndef DICE_H
#define DICE_H

#include "arbor.h"

Arbor_Game dice_copy(Arbor_Game game);
void dice_delete(Arbor_Game game);
void dice_make(Arbor_Game game, int action);
int dice_actions(Arbor_Game game);
int dice_side(Arbor_Game game);
int dice_eval(Arbor_Game game);
Arbor_Game dice_new(void);

#endif // DICE_H
