import re
from game import Game

# [] -> (4571,5038)
# [2] -> (4616,4962)
# [1,2] -> (4587,5010)
# [3,1,2] -> (4499,5066)
# [2,2,3,2] -> (4758,4832)

with open("data/set10.log", 'r') as f:
    i = 0
    for line in f:
        m = re.match(r"\[(.*?)\]\s*->\s*\((.*?)\)",line)

        if m:
            actions = m.group(1).split(',')
            p1, p2 = m.group(2).split(',')
            game = Game()

            for action in actions:
                game.make(action)
            
            print(game.show())
        else:
            print(f"error matching {line}")
    
        i += 1
        if i > 10: break

