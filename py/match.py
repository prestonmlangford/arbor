import multiprocessing
from game import Game
from tqdm import tqdm

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

if __name__ == '__main__':
    # p2 = {
    #     "name" : "andy",
    #     "path" : "target/release/reversi",
    #     "time" : 100
    # }

    p2 = {
        "name" : "link",
        "path" : "c/build/osx/bin/reversi",
        "time" : 100
    }

    p1 = {
        "name" : "master",
        "path" : "c/baseline/master",
        "time" : 100
    }

    # results: with fred expansion 10, andy expansion 0
    # fred 74
    # andy 109

    # results: with fred expansion 0, andy expansion 0
    # fred 88
    # andy 92

    p1_score = 0
    p1_name = p1["name"]
    p2_score = 0
    p2_name = p2["name"]

    for result in mp_tournament(p1,p2,5,8*10):
        r1, r2 = result
        if r1 == p1_name:
            p1_score += 1

        if r2 == p1_name:
            p1_score += 1

        if r1 == p2_name:
            p2_score += 1

        if r2 == p2_name:
            p2_score += 1
        
        print(result)

    print(f"{p1_name} {p1_score}")
    print(f"{p2_name} {p2_score}")