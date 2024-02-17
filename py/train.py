import json
import glob
import pandas
import numpy as np
from pathlib import Path
from sklearn import linear_model
from sklearn.model_selection import train_test_split
from sklearn.inspection import permutation_importance
from sklearn import metrics
import math

def sigmoid(x):
  return 1 / (1 + math.exp(-x))

def train(path):
    df = pandas.read_csv(path, header=None)
    arr = df.to_numpy()
    y = arr[:,0]
    x = arr[:,[1,2,3]]
    # x = arr[:,[1,2,3,4,5,6]]
    # x = arr[:,[8,9,10]]

    x_train, x_test, y_train, y_test = train_test_split(x, y, test_size=0.30, random_state=1)
    reg = linear_model.LogisticRegression()
    reg.fit(x_train, y_train)
    result = permutation_importance(reg,x_test,y_test)
    
    y_pred = reg.predict(x_test)
    count = 0
    for pred,test in zip(y_pred,y_test):
        if pred == test:
            count += 1
    f1 = metrics.f1_score(y_test, y_pred)
    acc = count/len(y_test)
    coef = []
    print(f"f1 = {f1:.3f}")
    print(f"acc = {acc:.3f}")
    
    print(" i: importance, value")
    for i,(m,s,c) in enumerate(zip(result.importances_mean,result.importances_std,reg.coef_[0])):
        coef.append(c)
        print(f"{i + 1:2d}: {100*m:.1f} {c:.3f}")
    print()
    # count = 0
    # w = reg.coef_
    # for x,y in zip(x_test,y_test):
    #     t = np.dot(w,x) + reg.intercept_
    #     if (t > 0) == y:
    #         count += 1

    # print(count/len(y_test))
    # print(reg.coef_[0])
    # for i,c in enumerate(reg.coef_[0].tolist()):
    #     print(i,c)
    # print()

    # return {
    #     "f1_score" : metrics.f1_score(y_test, y_pred),
    #     "test_accuracy" : count/len(y_test),
    #     "coefficients" : reg.coef_.tolist(),
    #     "samples" : len(y)
    # }
    return coef


def update_file(set,result):
    with open("data/logistic.json",'r') as f:
        j = json.load(f)
    
    j[set] = result

    with open("data/logistic.json",'w') as f:
        json.dump(j,f,indent=4)

if __name__ == '__main__':
    coefs = []
    for set in range(5,60):
        path = f"data/features/set{set}.csv"
        print(path)
        coef = train(path)
        coefs.append(coef)
        # print(result)
        # update_file(path.stem,result)
    
    for coef in coefs:
        c = ','.join(f"{f:>8.4f}" for f in coef)
        print(f"{{{c}}},")
