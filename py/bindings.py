import subprocess
import json

AI_C = 'c/a.out'

def run(bin,game,cmd):
    result = subprocess.run([bin] + game + [cmd], capture_output=True)
    # print(result.stderr.decode('utf-8'))
    return result.stdout.decode('utf-8').strip()

def show(game):
    return run(AI_C, game, "show")

def json_repr(game):
    
    return run(AI_C, game, "json")

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
