import subprocess
import random
from player import Player

BACKEND = Player("baseline", path="c/baseline/master")

class Game:
    def __init__(self,p1=BACKEND,p2=BACKEND) -> None:
        self.p1 = p1
        self.p2 = p2
        self.history = []

    def run(self,cmd,player=None):
        if player is None:
            player = BACKEND
        
        opt = []

        opt += [f"policy:{player.policy}"]

        c = [player.path] + opt + self.history + [cmd]
        result = subprocess.run(c, capture_output=True)
        return result.stdout.decode('utf-8').strip()

    def dump(self):
        actions = ','.join(self.history)
        return f"[{actions}]"

    def show(self):
        return self.run("show")

    def vector(self):
        return self.run("vector")

    def choose(self, player):
        return self.run(f"mcts:time:{player.time}", player)
        # return self.run(f"mcts:iter:{time*100}", player)

    def side(self):
        return self.run("side")

    def rollout(self,iterations):
        return self.run(f"rollout:{iterations}")

    def actions(self):
        return self.run("actions")

    def outcome(self):
        return self.run("result")

    def make(self, action):
        self.history.append(action)

    def unmake(self):
        self.history.pop()

    def swap(self):
        tmp = self.p1
        self.p1 = self.p2
        self.p2 = tmp

    def revert(self, depth):
        self.history = self.history[:depth]

    def random_start(self,depth):
        self.revert(0)
        for _ in range(depth):
            # try again if it gets a game over
            if self.side() == "none":
                self.random_start(depth)
                break

            count = self.actions()
            a = random.randint(0, int(count) - 1)
            self.make(str(a))

        

    def play_match(self):
        while True:
            # print(show(game) + '\n')

            if self.side() == "p1":
                p = self.p1
            elif self.side() == "p2":
                p = self.p2
            else:
                break

            action = self.choose(p)
            self.make(action)

        result = self.outcome()

        if result == "p1":
            winner = self.p1.name
        elif result == "p2":
            winner = self.p2.name
        else:
            winner = "draw"

        return winner
