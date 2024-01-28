
#ifndef BAD_BATTLESHIP_H
#define BAD_BATTLESHIP_H

#include "arbor.h"

typedef struct Bad_Battleship_t
{
    uint16_t p1_pins;
    uint16_t p2_pins;
    int result;
    int side;
} BB;

Arbor_Game bb_copy(Arbor_Game game);
void bb_free(Arbor_Game game);
void bb_make(Arbor_Game game, int action);
int bb_actions(Arbor_Game game);
int bb_side(Arbor_Game game);
int bb_eval(Arbor_Game game);
Arbor_Game bb_new(void);

#endif // BAD_BATTLESHIP_H
