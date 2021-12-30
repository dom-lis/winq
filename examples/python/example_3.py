from _cmn import parse_size, flush_screen
from collections import deque
from random import randrange

def rand_line(l, w):
    return ''.join(f'{randint(15):1x}' for _ in range(w))

def rand_field(l, w, h):
    return [''.join(f'{randint(15):1x}' for _ in range(w)) for _ in range(h)]

# presumably first input from tulip is terminal size
w, h = parse_size(input())

xs = deque(h * [''], h)

while True:
    i = input()
    if i == 'key:Esc':
        break
    xs.append(i)
    flush_screen(
        [x.ljust(w) for x in xs],
        [''.join(f'{randrange(16):1x}' for _ in range(w)) for _ in range(h)],
        [''.join(f'{randrange(16):1x}' for _ in range(w)) for _ in range(h)],
        [''.join(f'{randrange(4):1x}'  for _ in range(w)) for _ in range(h)],
    )
