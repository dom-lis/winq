def parse_size(s):
    a, b = s.split(':')
    assert a == 'size'
    x, y = [int(i) for i in b.split(',')]
    return x, y

def send(x, ls):
    j = '\n'.join(ls)
    print(f'\t{x}\n{j}')

def flush_lines(ls):
    send('text', ls)
    print('\tflush')

def flush_line(l):
    flush_lines([l])

def flush_screen(ls, bs, fs, ss):
    send('text',    ls)
    send('fg',      fs)
    send('bg',      bs)
    send('style',   ss)
    print('\tflush')
