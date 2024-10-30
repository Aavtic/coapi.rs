import socket
import threading
import os
import json
import queue
import sys


IPC_SOCK = "./ipc.sock"
RUNNER = "./runner.py"

try:
    os.unlink(IPC_SOCK)
except OSError:
    if os.path.exists(IPC_SOCK):
        raise


class Server:
    def __init__(self):
        self.running = True

    def serve(self):
        self.s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.s.bind(IPC_SOCK)
        self.s.listen()

        while self.running:
            print("Server listening...")
            conn, addr = self.s.accept()
            print("Connection from ", addr)
            threading.Thread(target=self.handle_client, args=(conn,)).start()

    def get_output(self, json_request, que):
        sin = Stdin(que)
        sys.stdin = sin
        print("Running...")
        exec(json_request["code"])

    def handle_client(self, conn):
        que = queue.Queue(maxsize=100)
        get_userinput = False
        json_msg = ""
        msg_buffer = ""
        try:
            while True:
                msg = conn.recv(1024).decode()
                print("received", msg)
                if msg:
                    msg_buffer += msg
                    try:
                        # print("trying: ", [i for i in msg_buffer])
                        msg_buffer = msg_buffer.split("\x00")[0]
                        print(msg_buffer)
                        json_msg = json.loads(msg_buffer)
                    except Exception as e:
                        print(e)
                        continue
                    if not get_userinput:
                        threading.Thread(target=self.get_output, args=(json_msg, que,)).start()
                        msg_buffer = ""
                        get_userinput = True
                    else:
                        print("got userinput", json_msg)
                        que.put(json_msg)
                        msg_buffer = ""

                else:
                    break
        finally:
            print("closing")
            conn.close()
            os.unlink(IPC_SOCK)


class Stdin:
    def __init__(self, que):
        sys.stdin = self
        self.que = que

    def exec(self, code: str):
        exec(code)
        self.que.put("Done")

    def readline(self):
        data = self.que.get(block=True)
        stdin_data = data["stdin"]

        return stdin_data


if __name__ == "__main__":
    server = Server()
    server.serve()
