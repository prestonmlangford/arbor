#include <stdio.h>
#include <stdint.h>
#include "arbor.h"

struct Silly_Game
{
    uint32_t seed;
    int score;
};

static uint32_t random(uint32_t seed)
{
  seed ^= seed << 13;
  seed ^= seed >> 17;
  seed ^= seed << 5;
  return seed;
}

int arbor_game_actions(Arbor_Game game)
{
    return 0;
}

bool arbor_game_make(Arbor_Game game, int action)
{
    return false;
}

bool arbor_game_side(Arbor_Game game)
{
    return true;
}

enum Arbor_Game_Result arbor_game_over(Arbor_Game game)
{
    return ARBOR_GAME_RESULT_DRAW;
}

int main (int argc, char* argv[])
{
    printf("Arbor\n");
    return 0;
}
