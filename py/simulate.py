from time import sleep
from multiprocessing import Pool, Queue
from game import Game
import os

MAX_GAME_LEN = 60
ITERATIONS = 10000
BACKEND = {"path" : "c/build/release/bin/reversi"}


def worker(depth):
    pid = os.getpid()
    game = Game(BACKEND,BACKEND)
    game.random_start(depth)
    result = game.rollout(ITERATIONS)
    print(f"{pid} rollout depth {depth} -> {game.dump()} ({result})")
    # sleep(0.333*(MAX_GAME_LEN - depth)/MAX_GAME_LEN)

if __name__ == '__main__':
    

    with Pool() as pool:
        for i in range(200):
            pool.map(worker,range(MAX_GAME_LEN))