#include "reversi_bb.h"
#include <assert.h>

/*
  ---------------------------------
7 | x | x | x | x | x | x | x | x |
  ---------------------------------
6 | x | x |   |   |   |   | x | x |
  ---------------------------------
5 | x |   | x |   |   | x |   | x |
  ---------------------------------
4 | x |   |   | x | x |   |   | x |
  ---------------------------------
3 | x |   |   | x | x |   |   | x |
  ---------------------------------
2 | x |   | x |   |   | x |   | x |
  ---------------------------------
1 | x | x |   |   |   |   | x | x |
  ---------------------------------
0 | x | x | x | x | x | x | x | x |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/
void ut_bb_corner_stability(void)
{
    {
        bb t = BB(0,0);
        bb b = bb_corner_stability(t);
        assert(bb_popcount(b) == 1);
    }

    {
        bb t = BB(0,0) | BB(1,0) | BB(1,1) | BB(0,1);
        bb b = bb_corner_stability(t);
        assert(bb_popcount(b) == 4);
    }

    {
        bb t = BB(1,0) | BB(1,1) | BB(0,1);
        bb b = bb_corner_stability(t);
        assert(bb_popcount(b) == 0);
    }

    {
        bb t = BB(0,0) | BB(1,0) | BB(0,1) | OUTSIDES;
        bb b = bb_corner_stability(t);
        assert(bb_popcount(b) == 11);
    }

    {
        bb t = SW | CORNERS_ADJACENT | OUTSIDES | DIAGONALS;
        bb b = bb_corner_stability(t);
        int count = bb_popcount(b);
        assert(count == 19);
    }

    {
        bb t = NW | CORNERS_ADJACENT | OUTSIDES | DIAGONALS;
        bb b = bb_corner_stability(t);
        int count = bb_popcount(b);
        assert(count == 19);
    }

    {
        bb t = NE | CORNERS_ADJACENT | OUTSIDES | DIAGONALS;
        bb b = bb_corner_stability(t);
        int count = bb_popcount(b);
        assert(count == 19);
    }

    {
        bb t = SE | CORNERS_ADJACENT | OUTSIDES | DIAGONALS;
        bb b = bb_corner_stability(t);
        int count = bb_popcount(b);
        assert(count == 19);
    }

    {
        bb t = CORNERS | CORNERS_ADJACENT | OUTSIDES | DIAGONALS;
        bb b = bb_corner_stability(t);
        int count = bb_popcount(b);
        assert(count == 40);
    }
}

int main (int argc, char* argv[])
{
    ut_bb_corner_stability();
    return 0;
}
