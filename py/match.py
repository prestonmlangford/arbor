import sys
import json
import multiprocessing
from game import Game
from tqdm import tqdm
from player import Player

def pair_match(info):
    p1,p2,depth = info

    game = Game(p1,p2)

    game.random_start(depth)
    round_1 = game.play_match()

    game.swap()
    game.revert(depth)
    round_2 = game.play_match()

    return round_1, round_2

def tournament(p1,p2,start,rounds):
    for _ in range(rounds):
        yield pair_match((p1,p2,start))

def mp_tournament(p1,p2,start,rounds):
    with multiprocessing.Pool() as pool:
        match_info = [(p1,p2,start)]*rounds
        it = pool.imap_unordered(pair_match, match_info)

        # tqdm will display a progress bar while this work is being done
        results = list(tqdm(it, total=rounds))

        return results

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


def main():

    cfg = json.loads(sys.argv[1])
    p1 = Player.from_cfg(cfg["p1"])
    p2 = Player.from_cfg(cfg["p2"])
    rounds = cfg.get("rounds",10)
    cores = multiprocessing.cpu_count()
    start = cfg.get("start",5)

    for r1, r2 in mp_tournament(p1,p2,start,rounds*cores):
        if r1 == p1.name:
            p1.score += 1

        if r2 == p1.name:
            p1.score += 1

        if r1 == p2.name:
            p2.score += 1

        if r2 == p2.name:
            p2.score += 1

    print(p1)
    print(p2)

if __name__ == '__main__':
    main()
    # print(sys.argv[1])