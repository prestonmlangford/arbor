
#ifndef REVERSI_BB_H
#define REVERSI_BB_H

#include <stdint.h>
#include "reversi_coef.h"

typedef uint64_t bb;

#define BB(y,x) (UINT64_C(1) << (((y) << 3) | (x)))

int bb_popcount(bb u);
bb bb_generate_moves(bb f, bb e);
bb bb_make_capture(bb f, bb e, bb c);
bb bb_mobility(bb f, bb e);
void bb_vector(bb f, bb e, float v[NUM_FEAT]);
void bb_show(bb x, bb o, bb d);

#endif // REVERSI_BB_H
