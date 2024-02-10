import subprocess
import random

class Game:
    def __init__(self,p1,p2) -> None:
        self.p1 = p1
        self.p2 = p2
        self.history = []

    def run(self,cmd,driver=None):
        if driver is None:
            driver = self.p1

        c = [driver["path"]] + self.history + [cmd]
        result = subprocess.run(c, capture_output=True)
        return result.stdout.decode('utf-8').strip()

    def dump(self):
        actions = ','.join(self.history)
        return f"[{actions}]"

    def show(self):
        return self.run("show")

    def choose(self, player):
        time = player["time"]
        return self.run(f"mcts:time:{time}", player)

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
            winner = self.p1["name"]
        elif result == "p2":
            winner = self.p2["name"]
        else:
            winner = "draw"

        return winner