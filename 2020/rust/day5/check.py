import sys

def bin2num(f):
  f = f.replace('F', '0')
  f = f.replace('B', '1')
  f = f.replace('R', '1')
  f = f.replace('L', '0')
  return int(f, base=2)

seats = []

with open(sys.argv[1]) as f:
    for line in f:
        seats.append(bin2num(line.strip()))


def part2(seats):
    missing = set()

    for seat in seats:
        missing.add(seat+1)
        missing.add(seat-1)

    opts = (set(missing)-set(seats))
    opts.remove(max(opts))
    opts.remove(min(opts))
    print('Your seat is:')
    print(list(opts)[0])

def part1(seats):
  print('The highest seat is:')
  print(max(seats))

part1(seats)
part2(seats)


