import traceback
from subprocess import Popen, PIPE
from threading import Thread
from queue import Queue, Empty
from time import sleep

subp = Popen(['cargo', 'run'], stdin=PIPE, stdout=PIPE)
stdin = Queue()
stdout = Queue()


def read_subp_stdin():
    for line in iter(subp.stdout.readline, b''):
        try:
            line = line.decode('utf-8')
            line = line.replace('\n', '')
            stdin.put(line)
        except:
            continue
    subp.stdout.close()


def write_subp_stdout():
    for buff in iter(stdout.get, None):
        for line in buff:
            line = line + '\n'
            line = line.encode('utf-8')
            subp.stdin.write(line)
        subp.stdin.flush()
    subp.stdin.close()


Thread(target=read_subp_stdin, daemon=True).start()
Thread(target=write_subp_stdout, daemon=True).start()

size_w = 0
size_h = 0
while True:
    try:
        m = stdin.get()
        m, *ps = m.split(':')
        ps = (ps or [''])[0].split(',')

        if m == 'Size':
            w, h = [int(s) for s in ps]
            size_w = w
            size_h = h

        elif m == 'MouseMove':
            x, y = [int(s) for s in ps]
            buff = [[' ' for _ in range(w)] for _ in range(h)]
            buff[y][x] = '@'
            buff = [''.join(line) for line in buff]
            buff = ['\ttext'] + buff + ['\tflush']
            stdout.put(buff)

    except Empty:
        break
    except KeyboardInterrupt:
        break
    except:
        print(traceback.format_exc())

