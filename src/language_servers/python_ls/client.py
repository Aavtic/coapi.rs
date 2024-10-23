import socket


IPC_SOCK = "./ipc.sock"


class Client:
    def __init__(self):
        self.s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.s.connect(IPC_SOCK)
        message = "Hello Server!"
        self.s.sendall(message.encode())

        response = self.s.recv(1024)
        print("Response : ", response.decode())

        self.s.close()


if __name__ == "__main__":
    Client()
