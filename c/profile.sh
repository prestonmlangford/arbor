gcc -lprofiler main.c arbor.c random.c profile.c bad_battleship.c dice.c rps.c reversi.c \
&& ./a.out 54 42 22 42 mcts:5000 \
&& ppro a.out
