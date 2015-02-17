#!/usr/bin/env python2
from __future__ import print_function

import gevent.monkey
#gevent.monkey.patch_all()
#gevent.monkey.patch_sys(stdin=True, stdout=False, stderr=False)

import gevent
import gevent.pool
import gevent.queue
import gevent.subprocess

import sys
import shlex
import signal
import os.path
from datetime import datetime

import logging


is_64bits = sys.maxsize > 2**32


class RsyncBackup(object):
    def __init__(self, source, dest):
        self.source = source
        self.dest = dest

    def pre_backup(self):
        pass

    def backup(self):
        dest = os.path.join(self.dest, 'last')
        gevent.subprocess.call(['rsync', '-a', '--del', self.source, dest])

    def _post_backup(self):
        last = os.path.join(self.dest, 'last')
        path = os.path.join(
            self.dest, 'backup-{}.tar'.format(datetime.now().isoformat())
        )
        p1 = gevent.subprocess.Popen(['nice', '-n', '19', 'tar', 'cf', path, last])
        p1.communicate()
        p2 = gevent.subprocess.Popen(['nice', '-n', '19', 'xz', '-e9', path])
        p2.communicate()


class Minecraft(object):
    def __init__(self, config):
        self.config = config

        self._pool = gevent.pool.Group()
        self._message_queue = gevent.queue.Queue()

        self._backup = RsyncBackup(config['path'], config['backup'])

        def stop(num, frame):
            self.stop()

        signal.signal(signal.SIGINT, stop)
        signal.signal(signal.SIGTERM, stop)

    @property
    def arguments(self):
        m = [
            '-server',
            '-Xms{}'.format(self.config['minmem']),
            '-Xmx{}'.format(self.config['maxmem'])
        ]

        if is_64bits:
            m.append('-d64')

        m.extend(shlex.split(self.config.get('args', '')))

        return m

    def run(self):
        cmd = self.arguments
        cmd.insert(0, self.config['java'])
        cmd.extend(['-jar', self.config['jar'], 'nogui'])

        self._process = gevent.subprocess.Popen(
            cmd, cwd=self.config.get('path'), universal_newlines=True,
            stdin=gevent.subprocess.PIPE, stdout=sys.stdout, stderr=sys.stderr
        )

        self._pool.spawn(self._reader_greenlet)
        self._pool.spawn(self._writer_greenlet)
        self._pool.spawn(self._backup_greenlet, False)
        self._process.wait()
        self._pool.kill()

    def _reader_greenlet(self):
        while True:
            gevent.socket.wait_read(sys.stdin.fileno())
            msg = sys.stdin.readline()
            if not msg:
                break

            self._message_queue.put(msg)

    def _writer_greenlet(self):
        while True:
            msg = self._message_queue.get()
            self._process.stdin.write(msg)
            self._process.stdin.flush()

    def _backup_greenlet(self, do=True):
        hours = int(self.config.get('interval'))
        if hours <= 0:
            logging.info('Backups disabled')
            return
        seconds = hours*60*60

        if do:
            self._do_backup()

        g = gevent.spawn_later(seconds, self._backup_greenlet)
        self._pool.add(g)

    def _do_backup(self):
        # make sure this will not run in parallel
        logging.info('Backup started')
        if self._process.poll() is not None:
            logging.warn('Server is not running, skipping backup!')
            return

        self._backup.pre_backup()

        if not self.config.get('quiet', False):
            self._message_queue.put('say [mcw] Starting backup.\n')
        self._message_queue.put('save-all\n')
        self._message_queue.put('save-off\n')

        while not self._message_queue.empty():
            gevent.sleep(0.1)

        try:
            self._backup.backup()
        finally:
            self._message_queue.put('save-on\n')
            self._message_queue.put('save-all\n')

        if not self.config.get('quiet', False):
            self._message_queue.put('say [mcw] Backup finished.\n')

        self._backup._post_backup()

        logging.info('Backup finished')

    def stop(self):
        self._message_queue.put('stop\n')


class Spigot(Minecraft):
    @property
    def arguments(self):
        m = Minecraft.arguments.fget(self)
        m.append('-XX:MaxPermSize=128M')
        return m


class Ftb(Minecraft):
    @property
    def arguments(self):
        m = Minecraft.arguments.fget(self)
        m.extend([
            '-XX:PermSize=256m', '-XX:+UseParNewGC',
            '-XX:+CMSIncrementalPacing', '-XX:+CMSClassUnloadingEnabled',
            '-XX:ParallelGCThreads=2', '-XX:MinHeapFreeRatio=5',
            '-XX:MaxHeapFreeRatio=10'
        ])
        return m


_REQUIRED_KEYS = (
    'type', 'java', 'jar', 'minmem', 'maxmem', 'path'
)


def main():
    logging.basicConfig(level=logging.DEBUG)

    import ConfigParser as configparser
    import argparse

    parser = argparse.ArgumentParser('mcw')
    parser.add_argument(
        'config', type=argparse.FileType('r'), help='Path to configfile'
    )
    parser.add_argument(
        'name', help='Config section for this server'
    )
    ns = parser.parse_args()

    config = configparser.ConfigParser()
    config.readfp(ns.config)
    config = dict(config.items(ns.name))

    cls = {
        'minecraft': Minecraft,
        'spigot': Spigot,
        'ftb': Ftb
    }[config['type'].strip().lower()]

    c = cls(config)
    c.run()


if __name__ == '__main__':
    main()
