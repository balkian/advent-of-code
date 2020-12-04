import re

passports = []
with open('input.txt') as f:
    passport = {}
    for line in f:
        if 'text' not in passport:
            passport['text'] = ''
        passport['text'] += line.strip().strip() + ' '
        if not line.strip():
            passports.append(passport)
            passport = {}
        tokens = line.split()
        for tok in tokens:
            k, v = tok.split(':')
            passport[k] = v
passports.append(passport)

def is_valid(p):
    valid = True
    for i in [
            'byr',
            'iyr',
            'eyr',
            'hgt',
            'hcl',
            'ecl',
            'pid',
    ]:
        if i not in p:
            valid = False
    return valid

COLORS = 'amb blu brn gry grn hzl oth'.split()
RGB = re.compile(r'#[a-z0-9]{6}')

def is_valid2(p):
    byr = p.get('byr')
    if not byr or (byr > '2002') or (byr < '1920'):
        return False
    eyr = p.get('eyr')
    if not eyr or (eyr > '2030') or (eyr < '2020'):
        return False
    iyr = p.get('iyr', None)
    if not iyr or (iyr > '2020') or (iyr < '2010'):
        return False

    if 'hgt' not in p:
        return False

    try:
        h = int(p['hgt'][:-2])
    except:
        return False

    units = p['hgt'][-2:]
    min, max = 0,0
    if units == 'cm':
        min, max = 150, 193
    elif units == 'in':
        min, max = 59, 76
    else:
        return False

    if h < min or h > max:
        return False
    if 'ecl' not in p or p['ecl'] not in COLORS:
        return False

    if 'pid' not in p or len(p['pid']) != 9:
        return False
    try:
        pid = int(p['pid'])
        if pid < 0 or pid >= 10**9:
            return False
    except:
        return False
    if not RGB.match(p.get('hcl', '')):
        return False

    return True

valid = [p for p in passports if is_valid(p)]
valid2 = [p for p in passports if is_valid2(p)]

# for p in passports:
#     if p not in valid:
#         for k,v in p.items():
#           print(f'{k}: {v}')
#         print()

print('Part 1', len(valid), len(passports))
print('Part 2', len(valid2), len(passports))
