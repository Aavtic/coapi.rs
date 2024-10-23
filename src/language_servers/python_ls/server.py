import socket
import os


IPC_SOCK = "./ipc.sock"

try:
    os.unlink(IPC_SOCK)
except OSError:
    if os.path.exists(IPC_SOCK):
        raise


class Server:
    def __init__(self):
        pass

    def serve(self):
        self.s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.s.bind(IPC_SOCK)
        self.s.listen()

        print("Server listening...")
        conn, addr = self.s.accept()
        print("Connection from ", addr)

        try:
            while True:
                msg = conn.recv(1024)
                if msg:
                    print("Msg: ", msg.decode())
                    conn.sendall("Hello Client!".encode())
                else:
                    break
        finally:
            conn.close()
            os.unlink(IPC_SOCK)


if __name__ == "__main__":
    server = Server()
    server.serve()
