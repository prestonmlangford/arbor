from multiprocessing import Pool
import re
from tqdm import tqdm
from game import Game
from pathlib import Path

def mp_worker(row):
    m = re.match(r"\[(.*?)\]\s*->\s*\((.*?)\)",row)
    if m:
        actions = m.group(1).split(',')
        p1, p2  = m.group(2).split(',')

        p1 = int(p1)
        p2 = int(p2)

        game = Game()
        for action in actions:
            game.make(action)

        x = game.vector()
        y = int(p1 > p2)

        return f"{y},{x}\n"
    else:
        return "\n"

def mp_features(rows):
    with Pool() as pool:
        it = pool.imap_unordered(mp_worker, rows)

        # tqdm will display a progress bar while this work is being done
        results = list(tqdm(it, total=len(rows)))

        return results

def process_raw_data(path):
    with open(path, 'r') as f:
        return mp_features(f.readlines())


def write_to_file(path,results):
    with open(path,'w') as f:
        for data in results:
            f.write(data)

if __name__ == '__main__':
    for set in range(60):
        path = f"data/raw/set{set}.log"
        results = process_raw_data(path)

        path = f"data/features/set{set}.csv"
        write_to_file(path, results)
