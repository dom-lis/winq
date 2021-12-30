from _cmn import flush_line

for i in range(0, 2 ** 24):
    flush_line(f'{i:24b}')
