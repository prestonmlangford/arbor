
#ifndef REVERSI_F1_H
#define REVERSI_F1_H
#include "reversi_bb.h"

/*
Q5
  ---------------------------------
7 |   |   |   |   |   |   |   |   |
  ---------------------------------
6 |   |   |   |   |   |   |   |   |
  ---------------------------------
5 |   |   |   |   |   |   |   |   |
  ---------------------------------
4 |   |   |   |   |   |   |   |   |
  ---------------------------------
3 |   |   |   |   |   |   |   |   |
  ---------------------------------
2 |   |   |   |   |   |   |   |   |
  ---------------------------------
1 |   |   |   |   |   |   |   |   |
  ---------------------------------
0 | x |   |   |   |   |   |   |   |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/
#define L00 BB(0,0)
#define L10 BB(7,0)
#define L20 BB(0,7)
#define L30 BB(7,7)

/*
Q4
  ---------------------------------
7 |   |   |   |   |   |   |   |   |
  ---------------------------------
6 |   |   |   |   |   |   |   |   |
  ---------------------------------
5 |   |   |   |   |   |   |   |   |
  ---------------------------------
4 |   |   |   |   |   |   |   |   |
  ---------------------------------
3 |   |   |   |   |   |   |   |   |
  ---------------------------------
2 |   |   |   |   |   |   |   |   |
  ---------------------------------
1 | x | x |   |   |   |   |   |   |
  ---------------------------------
0 |   | x |   |   |   |   |   |   |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/
#define L01 (BB(1,0) | BB(1,1) | BB(0,1))
#define L11 (BB(6,0) | BB(6,1) | BB(7,1))
#define L21 (BB(1,7) | BB(1,6) | BB(0,6))
#define L31 (BB(6,7) | BB(6,6) | BB(7,6))

/*
Q4
  ---------------------------------
7 |   |   |   |   |   |   |   |   |
  ---------------------------------
6 |   |   |   |   |   |   |   |   |
  ---------------------------------
5 |   |   |   |   |   |   |   |   |
  ---------------------------------
4 |   |   |   |   |   |   |   |   |
  ---------------------------------
3 |   |   |   |   |   |   |   |   |
  ---------------------------------
2 | x | x | x |   |   |   |   |   |
  ---------------------------------
1 |   |   | x |   |   |   |   |   |
  ---------------------------------
0 |   |   | x |   |   |   |   |   |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/
#define L02 (BB(2,0) | BB(2,1) | BB(2,2) | BB(1,2) | BB(0,2))
#define L12 (BB(5,0) | BB(5,1) | BB(5,2) | BB(6,2) | BB(7,2))
#define L22 (BB(2,7) | BB(2,6) | BB(2,5) | BB(1,5) | BB(0,5))
#define L32 (BB(5,7) | BB(5,6) | BB(5,5) | BB(6,5) | BB(7,5))

/*
Q4
  ---------------------------------
7 |   |   |   |   |   |   |   |   |
  ---------------------------------
6 |   |   |   |   |   |   |   |   |
  ---------------------------------
5 |   |   |   |   |   |   |   |   |
  ---------------------------------
4 |   |   |   |   |   |   |   |   |
  ---------------------------------
3 | x | x | x | x |   |   |   |   |
  ---------------------------------
2 |   |   |   | x |   |   |   |   |
  ---------------------------------
1 |   |   |   | x |   |   |   |   |
  ---------------------------------
0 |   |   |   | x |   |   |   |   |
  ---------------------------------
    0   1   2   3   4   5   6   7
*/
#define L03 (BB(3,0) | BB(3,1) | BB(3,2) | BB(3,3) | BB(2,3) | BB(1,3) | BB(0,3))
#define L13 (BB(4,0) | BB(4,1) | BB(4,2) | BB(4,3) | BB(5,3) | BB(6,3) | BB(7,3))
#define L23 (BB(3,7) | BB(3,6) | BB(3,5) | BB(3,4) | BB(2,4) | BB(1,4) | BB(0,4))
#define L33 (BB(4,7) | BB(4,6) | BB(4,5) | BB(4,4) | BB(5,4) | BB(6,4) | BB(7,4))

#define F1_P(f,e,n)\
({\
    float _f = (float) bb_popcount(f);\
    float _e = (float) bb_popcount(e);\
    0.5*(1.0 + (_f - _e)/((float) n));\
})

#define FEATURE_SET_2(f,e)\
{\

}

#endif // REVERSI_F1_H
