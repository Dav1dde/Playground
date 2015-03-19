from utopia.client import EasyClient, Identity
from utopia.plugins.util import LogPlugin
from utopia import signals

import logging
import argparse
import sys


def main():
    parser = argparse.ArgumentParser('idlebot')
    parser.add_argument(
        '-c', '--channel', nargs='+', dest='channels',
        default=[], help='List of channels'
    )
    parser.add_argument(
        '--debug', action='store_true'
    )
    parser.add_argument(
        'username', help='Twitch username'
    )
    parser.add_argument(
        'password', help='Oauth password for twitch chat'
    )
    ns = parser.parse_args()
    
    if ns.debug:
        logging.basicConfig(
            format='[%(asctime)s] %(levelname)s:\t%(message)s',
            datefmt='%m/%d/%Y %H:%M:%S', level=logging.DEBUG
        )
    
    ident = Identity(ns.username, password=ns.password)
    client = EasyClient(ident, 'irc.twitch.tv', port=6667, plugins=[LogPlugin()])
    
    def join(*args, **kwargs):
        for channel in ns.channels:
            client.join_channel(channel)
    
    signals.on_registered.connect(join, sender=client)

    client.connect().get()
    client._io_workers.join()   


if __name__ == '__main__':
    main()


