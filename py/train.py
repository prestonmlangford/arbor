import json
import glob
import pandas
import numpy as np
from pathlib import Path
from sklearn import linear_model
from sklearn.model_selection import train_test_split 
from sklearn import metrics
import math

def sigmoid(x):
  return 1 / (1 + math.exp(-x))

def train(path):
    df = pandas.read_csv(path, header=None)
    arr = df.to_numpy()
    y = arr[:,0]
    x = arr[:,[1,4,5,6,7,8]]

    x_train, x_test, y_train, y_test = train_test_split(x, y, test_size=0.30, random_state=1)
    reg = linear_model.LogisticRegression()
    reg.fit(x_train, y_train)
    
    y_pred = reg.predict(x_test)
    count = 0
    for pred,test in zip(y_pred,y_test):
        if pred == test:
            count += 1
    print(metrics.f1_score(y_test, y_pred))
    print(count/len(y_test))
    
    # count = 0
    # w = reg.coef_
    # for x,y in zip(x_test,y_test):
    #     t = np.dot(w,x) + reg.intercept_
    #     if (t > 0) == y:
    #         count += 1

    # print(count/len(y_test))
    # print(reg.coef_)
    print()

    # return {
    #     "f1_score" : metrics.f1_score(y_test, y_pred),
    #     "test_accuracy" : count/len(y_test),
    #     "coefficients" : reg.coef_.tolist(),
    #     "samples" : len(y)
    # }


def update_file(set,result):
    with open("data/logistic.json",'r') as f:
        j = json.load(f)
    
    j[set] = result

    with open("data/logistic.json",'w') as f:
        json.dump(j,f,indent=4)

if __name__ == '__main__':
    for s in glob.glob("data/trial_1/*.csv"):
        path = Path(s)
        train(path)
        # print(result)
        # update_file(path.stem,result)