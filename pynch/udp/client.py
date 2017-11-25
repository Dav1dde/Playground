import gevent.socket
import json


class BroadcastPunchClient(object):
    def __init__(self):
        self._buffer_size = 8192
        self._encoding = 'utf-8'
        self._listen = False 

        self.clients = set()
        self.socket = gevent.socket.socket(type=gevent.socket.SOCK_DGRAM)

    def handshake(self, address):
        self.socket.sendto(b'hi', address)
        clients = json.loads(self._receive()[0])

        self.clients = set((host, int(port)) for host, port in clients)

    def broadcast(self, message):
        for address in self.clients:
            self._send(address, message)

    def _send(self, address, message):
        with gevent.socket.socket(type=gevent.socket.SOCK_DGRAM) as s:
            print('---> {address[0]}:{address[1]} | {message}'.format(address=address, message=message))
            s.sendto(message.encode(self._encoding), address)

    def listen(self):
        self._listen = True
        while self._listen:
            data, address = self._receive()
            print('<--- {address[0]}:{address[1]} | {data}'.format(address=address, data=data))

    def stop(self):
        self._listen = False

    def _receive(self):
        data, address = self.socket.recvfrom(self._buffer_size)
        return data.decode(self._encoding), address


def _punch_broadcast(ns):
    c = BroadcastPunchClient()
    c.handshake((ns.host, ns.port))
    print('--- {address[0]}:{address[1]} ---'.format(address=c.socket.getsockname()))
    c.broadcast(ns.message)
    c.listen()


def main():
    import argparse

    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest='subparser_name')

    punch = subparsers.add_parser('punch')
    punch.add_argument('--host', default='0.0.0.0', help='Host of the UDP server')
    punch.add_argument('--port', default=6030, type=int, help='Port of the UDP server')
    punch.add_argument('--message', default='Hallo I bims, a client')

    ns = parser.parse_args()

    func = {
        'punch': lambda: _punch_broadcast(ns)
    }[ns.subparser_name]
    func()


if __name__ == '__main__':
    main()
