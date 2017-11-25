import gevent.server
import json
import time


class SimpleServer(gevent.server.DatagramServer):
    def handle(self, data, address):
        print('<--- {address[0]}:{address[1]} | {data!r}'.format(address=address, data=data))


class EchoServer(SimpleServer):
    def handle(self, data, address):
        SimpleServer.handle(self, data, address)

        self.sendto(data, address)


class PunchServer(SimpleServer):
    def __init__(self, *args, **kwargs):
        SimpleServer.__init__(self, *args, **kwargs)

        self._encoding = 'utf-8'

        self.clients = dict()

    def handle(self, data, address):
        SimpleServer.handle(self, data, address)

        self.clients[address] = time.time()

        data = json.dumps(list(self.clients.keys() - {address})).encode(self._encoding)
        self.sendto(data, address)


def main():
    import argparse

    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest='subparser_name')

    simple = subparsers.add_parser('simple')
    simple.add_argument('--host', default='0.0.0.0', help='Host of the UDP server')
    simple.add_argument('--port', default=6030, type=int, help='Port of the UDP server')

    echo = subparsers.add_parser('echo')
    echo.add_argument('--host', default='0.0.0.0', help='Host of the UDP server')
    echo.add_argument('--port', default=6030, type=int, help='Port of the UDP server')

    punch = subparsers.add_parser('punch')
    punch.add_argument('--host', default='0.0.0.0', help='Host of the UDP server')
    punch.add_argument('--port', default=6030, type=int, help='Port of the UDP server')

    ns = parser.parse_args()

    func = {
        'simple': lambda: SimpleServer((ns.host, ns.port)).serve_forever(),
        'echo': lambda: EchoServer((ns.host, ns.port)).serve_forever(),
        'punch': lambda: PunchServer((ns.host, ns.port)).serve_forever()
    }[ns.subparser_name]
    func()


if __name__ == '__main__':
    main()
