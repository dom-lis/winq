from subprocess import Popen, PIPE
from time import sleep
from datetime import datetime
from math import sin, floor

subp = Popen(['cargo', 'run'], stdin=PIPE)

def init_color(x, y):
    a = sin(x * 0.1)
    b = sin(y * 0.1)
    c = a * b
    c = c * 0.5 + 0.5
    c = floor(c * 16)
    return c

w = 128
h = 64
b = [[init_color(x, y) for x in range(w)] for y in range(h)]

def color(n):
    c = n % 16
    return f'{c:x}'

while True:
    sleep(0.1)
    if subp.poll() is not None:
        break
    b = [[n + 1 for n in line] for line in b]
    buff = '\n'.join([''.join([color(n) for n in line]) for line in b])
    subp.stdin.write(b'\tbg\n')
    subp.stdin.write(f'{buff}\n'.encode('utf-8'))
    subp.stdin.write(b'\tflush\n')
    subp.stdin.flush()

