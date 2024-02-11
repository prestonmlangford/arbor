
#ifndef REVERSI_H
#define REVERSI_H

#include "arbor.h"

Arbor_Game reversi_new(void);
void reversi_show(Arbor_Game game);
void reversi_vector(Arbor_Game game);
Arbor_Game reversi_copy(Arbor_Game game);
void reversi_delete(Arbor_Game game);

void reversi_make(Arbor_Game game, int action);
int reversi_actions(Arbor_Game game);
int reversi_side(Arbor_Game game);
int reversi_eval(Arbor_Game game);

#endif // REVERSI_H
