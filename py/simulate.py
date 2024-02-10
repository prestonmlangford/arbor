from time import sleep
from multiprocessing import Pool, Queue
from game import Game
import os

MAX_GAME_LEN = 60
ITERATIONS = 10000
BACKEND = {"path" : "c/build/release/bin/reversi"}


def worker(depth):
    game = Game(BACKEND,BACKEND)
    game.random_start(depth)
    result = game.rollout(ITERATIONS)
    return f"{game.dump()} -> ({result})\n"

if __name__ == '__main__':
    with Pool() as pool:
        batch = 1
        while True:
            with open("summary.log",'w') as f:
                f.write(f"cpu count: {os.cpu_count()}\n")
                f.write(f"batch: {batch}\n")

            results = pool.map(worker,range(MAX_GAME_LEN))

            with open("rollout.log",'a') as f:
                for result in results:
                    f.write(result)
            
            batch += 1
