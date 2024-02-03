import subprocess

SEARCH_TIME_MS = 1000
AI_C = 'c/a.out'
AI_R = 'target/release/reversi'

def run(bin,game,cmd):
    result = subprocess.run([bin] + game + [cmd], capture_output=True)
    print(result.stderr.decode('utf-8'))
    return result.stdout.decode('utf-8').strip()

def show(game):       return run(AI_C,   game, "show") + '\n'
def ai(game, player): return run(player, game, f"mcts:{SEARCH_TIME_MS}")
def side(game):       return run(AI_C,   game, "side")
def result(game):     return run(AI_C,   game, "result")

game = []
while True:
    print(show(game))

    if side(game) == "p1":
        player = AI_R
    elif side(game) == "p2":
        player = AI_C
    else:
        break

    game.append(ai(game,player))
