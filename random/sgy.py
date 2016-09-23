from itertools import chain

HEX = '0123456789ABCDEF'


def decode(inp):
    if len(inp) & 1:
        raise ValueError('Invalid length')

    indices = [HEX.index(c) for c in inp]
    return ''.join(chr(a << 4 | b) for a, b in zip(indices[::2], indices[1::2]))


def encode(inp):
    return ''.join(chain.from_iterable(
        (HEX[((c >> 4) & 0xF)], HEX[c & 0xF]) for c in map(ord, inp)
    ))


if __name__ == '__main__':
    s = '<-Test->'
    assert decode(encode(s)) == s

    print(encode(s))
