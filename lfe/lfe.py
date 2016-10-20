import argparse
import bz2
import os
import re
from collections import namedtuple
from itertools import chain
from urllib.parse import urlparse, parse_qsl, urlencode, urlunparse, quote

OPENERS = {
    '.bz2': bz2.open
}

DEFAULT_OPENER = open


def normalize_query(query):
    for k, v in query:
        if v and not v.isspace():
            yield (k, '@'.join(sorted(v.split('@'))))


def normalize_url(url):
    parsed = urlparse(url)
    qs = normalize_query(parse_qsl(parsed.query))
    # sort so the query, so the order never changes
    query = urlencode(sorted(qs), safe='/$@', quote_via=quote)
    # Rebuild original URL with stripped and sorted query
    return urlunparse((
        parsed.scheme,
        parsed.netloc,
        parsed.path,
        parsed.params,
        query,
        parsed.fragment
    ))


def make_regex_parser(regex, converter=None):
    regex = re.compile(regex)
    if converter is None:
        converter = lambda x: x

    def parse_line(line):
        try:
            return converter(regex.match(line).groups())
        except AttributeError:
            raise ValueError('Cannot parse line {!r}'.format(line))

    return parse_line


LogLine = namedtuple('LogLine', ['hostname', 'some_ip', 'identity', 'user_id', 'time',
                                 'request', 'status_code', 'size', 'referrer', 'user_agent'])
_DEFAULT_PARSER = make_regex_parser(
    r'([(\d\.)]+) "(.*)" ([^\s+]) ([^\s]+) \[(.*)\] "(.*)" (\d+) (\d+|-) "(.*)" "(.*)"'.replace(' ', '\s*'),
    lambda x: LogLine(*x)
)


def read_logfile(path, parser=_DEFAULT_PARSER, ignore_errors=False, quiet=True):
    if not quiet:
        print('Reading: ', path)

    _, ext = os.path.splitext(path)
    opener = OPENERS.get(ext, DEFAULT_OPENER)

    with opener(path, mode='rt') as f:
        for line in f:
            try:
                yield parser(line)
            except Exception:
                if not ignore_errors:
                    raise


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('files', nargs='+')
    parser.add_argument('-o', '--out-file', required=True)
    parser.add_argument('--quiet', action='store_true')
    ns = parser.parse_args()

    log_lines = chain.from_iterable(
        read_logfile(path, ignore_errors=False, quiet=ns.quiet) for path in ns.files
    )
    seen = set()

    with open(ns.out_file, 'w') as f:
        for line in log_lines:
            if not line.status_code == '200':
                continue
            request = line.request
            url = request.split(None, 1)[1].rsplit(None, 1)[0]
            url = normalize_url(url)

            if not url in seen:
                seen.add(url)
                f.write('{}\n'.format(url))


if __name__ == '__main__':
    main()
