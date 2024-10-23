import socket
import threading
import os
import json
import subprocess


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

    def get_output(self, json_request):
        code = json_request["code"]
        proc = subprocess.Popen([
            "python3",
            RUNNER,
            code
            ], stdout=subprocess.PIPE)
        out, err = proc.communicate()
        print(out, err)

    def handle_client(self, conn):
        json_msg = ""
        msg_buffer = ""
        try:
            while True:
                msg = conn.recv(1024).decode()
                if msg:
                    msg_buffer += msg
                    try:
                        json_msg = json.loads(msg)
                    except:
                        continue
                    print(json_msg)
                    self.get_output(json_msg)
                    msg_buffer = ""
                else:
                    break
        finally:
            conn.close()
            os.unlink(IPC_SOCK)


if __name__ == "__main__":
    server = Server()
    server.serve()
