from multiprocessing import Pool
import sys
import re
from tqdm import tqdm
from game import Game
from sklearn import linear_model
from sklearn.model_selection import train_test_split 
from sklearn import metrics

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

def unzip(lst):
    a = []
    b = []
    for x,y in lst:
        a.append(x)
        b.append(y)
    return (a,b)

def mp_worker(item):
    history, results = item
    actions = history.split(',')
    p1, p2 = results.split(',')

    p1 = int(p1)
    p2 = int(p2)

    game = Game()
    for action in actions:
        game.make(action)

    if game.side() == "p1":
        y = p1 > p2
    else:
        y = p2 > p1

    x = [float(v) for v in game.vector().split(',')]

    return (x, y)

def mp_features(data):
    with Pool() as pool:
        it = pool.imap_unordered(mp_worker, data)

        # tqdm will display a progress bar while this work is being done
        result = tqdm(it, total=len(data))

        return unzip(result)

def train(x,y):
    x_train, x_test, y_train, y_test = train_test_split(x, y, test_size=0.30, random_state=1)
    reg = linear_model.LogisticRegression()
    reg.fit(x_train, y_train)
    y_pred = reg.predict(x_test)
    f1 = metrics.f1_score(y_test, y_pred)

    count = 0
    for pred,test in zip(y_pred,y_test):
        if pred == test:
            count += 1

    print(f"f1 = {f1}")
    print(f"{count/len(y_test)}")
    print(reg.coef_)

if __name__ == '__main__':
    dataset = get_data(sys.argv[1])
    x_data, y_data = mp_features(dataset)
    train(x_data,y_data)
