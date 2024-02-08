
#ifndef RPS_H
#define RPS_H

#include "arbor.h"

Arbor_Game rps_copy(Arbor_Game game);
void rps_delete(Arbor_Game game);
void rps_make(Arbor_Game game, int action);
int rps_actions(Arbor_Game game);
int rps_side(Arbor_Game game);
int rps_eval(Arbor_Game game);
Arbor_Game rps_new(void);

#endif // RPS_H
