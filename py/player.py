
class Player:
    def __init__(self,
                 name,
                 path="c/baseline/master",
                 time=100,
                 policy="rollout") -> None:

        self.name = name
        self.path = path
        self.score = 0
        self.time = time
        self.policy = policy

    def __repr__(self) -> str:

        return (
            "-"*80 + "\n" + 
            f"name:   {self.name}\n" + 
            f"path:   {self.path}\n" + 
            f"time:   {self.time}\n" + 
            f"policy: {self.policy}\n" + 
            f"score:  {self.score}\n" +
            "-"*80 + "\n"
        )

    def from_cfg(cfg):
        return Player(cfg["name"],
                      path=cfg.get("path","c/baseline/master"),
                      time=cfg.get("time",100),
                      policy=cfg.get("policy","rollout"))
