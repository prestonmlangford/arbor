import subprocess

AI_C = 'c/a.out'
AI_R = 'target/release/reversi'

def show(history):
    result = subprocess.check_output([AI_C] + history + ["show"])
    print(result.decode('utf-8'))

def ai(path, history):
    result = subprocess.check_output([path] + history)
    return result.decode('utf-8').strip()

h = []
pair = AI_C, AI_R

show(h)

while True:
    x,y = pair
    a = ai(x,h)
    pair = y,x

    try:
        a = ai(x,h)
    except:
        # print(a)
        exit(-1)

    h.append(a)
    print(h)
    show(h)

    if a == 'white':
        break
    elif a == 'black':
        break
    elif a == 'draw':
        break
