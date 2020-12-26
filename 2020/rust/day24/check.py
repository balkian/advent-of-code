from collections import defaultdict

def parse(f):
    tiles = defaultdict(bool)
    for line in f:
        pos = [0, 0]
        line = iter(line)
        for c in line:
            if c == 'w':
                pos[0] += 2
            if c == 'e':
                pos[0] -= 2
            if c == 's':
                pos[1] += 1
                if next(line) == 'w':
                    pos[0] += 1
                else:
                    pos[0] -= 1
            if c == 'n':
                pos[1] -= 1
                if next(line) == 'w':
                    pos[0] += 1
                else:
                    pos[0] -= 1
        pos = tuple(pos)
        tiles[pos] = not tiles[pos]
        print(pos)
    print(tiles, len(tiles))
    print(len([t for t in tiles.values() if t]))
    return tiles

def game(tiles):

    for i in range(100):
        nt = defaultdict(bool)
        count = defaultdict(int)
        for (tile, v) in tiles.items():
            if not v:
                continue
            for d in [(-2,0), (+2,0), (1,1), (1,-1), (-1, 1), (-1, -1)]:
                count[(tile[0]+d[0], tile[1]+d[1])] += 1

        for (tile, count) in count.items():
            if count in [1, 2] and tiles[tile]:
                nt[tile] = True
            if count == 2 and not tiles[tile]:
                nt[tile] = True
        tiles = nt
        print(len(nt))

with open('input.txt') as f:
    tiles = parse(f)

game(tiles)
