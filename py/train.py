from multiprocessing import Pool
import sys
import math
import re
from tqdm import tqdm
from game import Game
import numpy as np
from sklearn import linear_model
from sklearn.model_selection import train_test_split 
from sklearn.metrics import mean_squared_error, r2_score

# [] -> (4571,5038)
# [2] -> (4616,4962)
# [1,2] -> (4587,5010)
# [3,1,2] -> (4499,5066)
# [2,2,3,2] -> (4758,4832)

def get_data(path):
    data = []

    with open(path, 'r') as f:
        for line in f:
            m = re.match(r"\[(.*?)\]\s*->\s*\((.*?)\)",line)

            if m:
                data.append((m.group(1),m.group(2)))

            else:
                print(f"error matching {line}")
    
    return data

def target(results):
    p1, p2 = results.split(',')
    
    p1 = int(p1)
    p2 = int(p2)

    return 0.5*(1.0 + (p1 - p2)/(p1 + p2))

def features(history):
    actions = history.split(',')
    
    game = Game()
    for action in actions:
        game.make(action)
    
    v = game.vector().split(',')

    return [float(w) for w in v]

def unzip(lst):
    a = []
    b = []
    for x,y in lst:
        a.append(x)
        b.append(y)
    return (a,b)

def mp_worker(item):
    history, results = item
    return (features(history), target(results))

def mp_features(data):
    with Pool() as pool:
        # tqdm will display a progress bar while this work is being done
        return unzip(tqdm(pool.imap_unordered(mp_worker, data), total=len(data)))

def train(x,y):
    x_train, x_test, y_train, y_test = train_test_split(x, y, test_size=0.30, random_state=1)
    reg = linear_model.LinearRegression()
    reg.fit(x_train, y_train)
    y_pred = reg.predict(x_test)
    mse = math.sqrt(mean_squared_error(y_test, y_pred))
    r2 = r2_score(y_test, y_pred)

    print(f"sd: {mse}")
    print(f"r2: {r2}")
    print(reg.coef_)

if __name__ == '__main__':
    dataset = get_data(sys.argv[1])
    x_data, y_data = mp_features(dataset)
    train(x_data,y_data)
