import subprocess
import random
import multiprocessing

AI_C = 'c/a.out'
AI_R = 'target/release/reversi'

def run(bin,game,cmd):
    result = subprocess.run([bin] + game + [cmd], capture_output=True)
    # print(result.stderr.decode('utf-8'))
    return result.stdout.decode('utf-8').strip()

def show(game):
    return run(AI_C, game, "show")

def choose(game, player):
    path = player["path"]
    time = player["time"]
    return run(path, game, f"mcts:time:{time}")

def side(game):
    return run(AI_C, game, "side")

def actions(game):
    return run(AI_C, game, "actions")

def outcome(game):
    return run(AI_C, game, "result")

def random_start(depth):
    game = []
    for _ in range(depth):
        count = actions(game)
        a = random.randint(0, int(count) - 1)
        game.append(str(a))

    # try again if it gets a game over
    if side(game) == "none":
        return random_start(depth)
    else:
        return game

def play_match(game,p1,p2):
    while True:
        # print(show(game) + '\n')

        if side(game) == "p1":
            p = p1
        elif side(game) == "p2":
            p = p2
        else:
            break

        game.append(choose(game,p))
        # print(".",end='', flush=True)

    result = outcome(game)
    
    if result == "p1":
        winner = p1["name"]
    elif result == "p2":
        winner = p2["name"]
    else:
        winner = "draw"
    
    print(winner)

    return winner

def pair_match(info):
    p1,p2,start = info

    game = random_start(start)
    round_1 = play_match(game, p1, p2)

    game = game[:start]
    round_2 = play_match(game, p2, p1)

    return round_1, round_2

def tournament(p1,p2,start,rounds):
    for _ in range(rounds):
        result = pair_match((p1,p2,start))
        print(result)

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
    p1 = {
        "name" : "andy",
        "path" : AI_R,
        "time" : 1000
    }

    p2 = {
        "name" : "fred",
        "path" : AI_C,
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
    for result in mp_tournament(p1,p2,5,100):
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