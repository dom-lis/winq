from subprocess import Popen, PIPE
from time import sleep
from datetime import datetime

subp = Popen('winq', stdin=PIPE)

while True:
    sleep(1)
    now = datetime.now().strftime('%d.%m.%Y %H:%M:%S')
    if subp.poll() is not None:
        break
    subp.stdin.write(f'{now}\n'.encode('utf-8'))
    subp.stdin.write(b'\tflush\n')
    subp.stdin.flush()
