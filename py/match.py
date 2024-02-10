import multiprocessing
from game import Game

def pair_match(info):
    p1,p2,depth = info

    game = Game(p1,p2)

    game.random_start(depth)
    round_1 = game.play_match()

    game.revert(depth)
    round_2 = game.play_match()

    return round_1, round_2

def tournament(p1,p2,start,rounds):
    for _ in range(rounds):
        yield pair_match((p1,p2,start))

def mp_tournament(p1,p2,start,rounds):
    match_info = [(p1,p2,start)]*rounds
    pool = multiprocessing.Pool()
    return pool.map(pair_match,match_info)

# Reversi Perft
# [DEPTH] [NUM LEAF NODES]
# ========================
#  1 4
#  2 12
#  3 56
#  4 244
#  5 1396
#  6 8200
#  7 55092
#  8 390216
#  9 3005288
# 10 24571284
# 11 212258800
# 12 1939886636
# 13 18429641748
# 14 184042084512
# 15 1891832540064
# 16 20301186039128

if __name__ == '__main__':
    p2 = {
        "name" : "andy",
        "path" : "target/release/reversi",
        "time" : 1000
    }

    p1 = {
        "name" : "fred",
        "path" : "c/build/release/bin/reversi",
        "time" : 1000
    }

    # results: with fred expansion 10, andy expansion 0
    # fred 74
    # andy 109

    # results: with fred expansion 0, andy expansion 0
    # fred 88
    # andy 92

    andy = 0
    fred = 0
    for result in mp_tournament(p1,p2,5,2):
        p1, p2 = result
        if p1 == "andy":
            andy += 1

        if p2 == "andy":
            andy += 1

        if p1 == "fred":
            fred += 1

        if p2 == "fred":
            fred += 1
        
        print(result)

    print(f"fred {fred}")
    print(f"andy {andy}")