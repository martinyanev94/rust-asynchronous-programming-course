import threading
import time
import socket

def handle_client(client_socket):
    request = client_socket.recv(1024)
    print(f"Received: {request.decode('utf-8')}")
    time.sleep(1)  # Simulate processing time
    client_socket.send("HTTP/1.1 200 OK\r\n\r\nHello World".encode('utf-8'))
    client_socket.close()

def start_server():
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.bind(('0.0.0.0', 9999))
    server_socket.listen(5)
    print("Listening on port 9999...")
    
    while True:
        client_socket, addr = server_socket.accept()
        print(f"Accepted connection from {addr}")
        client_handler = threading.Thread(target=handle_client, args=(client_socket,))
        client_handler.start()

start_server()
