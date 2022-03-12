from subprocess import Popen, PIPE
from threading import Thread
from queue import Queue, Empty
from time import sleep

subp = Popen('winq', stdin=PIPE, stdout=PIPE)
msg = Queue()

def read_subp_stdin():
    for line in iter(subp.stdout.readline, b''):
        msg.put(line)
    subp.stdout.close()

Thread(target=read_subp_stdin, daemon=True).start()

while True:
    sleep(2)
    while True:
        try:
            m = msg.get_nowait()
            print(f'msg: `{m}`')
        except Empty:
            pass
