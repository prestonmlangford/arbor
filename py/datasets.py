import re
import sys


# [] -> (4571,5038)
# [2] -> (4616,4962)
# [1,2] -> (4587,5010)
# [3,1,2] -> (4499,5066)
# [2,2,3,2] -> (4758,4832)

data = {}
for i in range(61):
    data[i] = ""

with open("data/rollout.log", 'r') as f:
    for line in f:
        m = re.match(r"\[(.*?)\]\s*->\s*\((.*?)\)",line)

        if m:
            try:
                actions = m.group(1).split(',')
                p1, p2 = m.group(2).split(',')
            except:
                print(line)
                continue

            # print(f"p1 = {p1}, p2 = {p2}, actions = {actions}")
            if actions[0] == '':
                count = 0
            else:
                count = len(actions)

            data[count] += line.rstrip() + '\n'
        else:
            print(f"error matching {line}")

for key,val in data.items():
    with open(f"data/set{key}.log",'w') as f:
        f.write(val)
