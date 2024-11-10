import queue
import sys

queue = queue.Queue(100)
STDIN_FILE = "./stdin"


class Stdin:
    def readline(que):
        while True:
            try:
                print(que.get(block=False))
            except:
                with open(STDIN_FILE, 'r') as f:
                    contents = f.read()
                    if len(contents) > 1:
                        return contents
                    else:
                        continue
            return


sin = Stdin()
sys.stdin = sin

import code

queue.put("Done")

