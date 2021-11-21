values = list()
r1 = r2 = r3 = r4 = r5 = 0
while True:
  r4 = r3 | 65536
  r3 = 2176960
  while True:
    r1 = r4 & 255

    r3 = r3 + r1
    # keep 24 bits
    r3 = r3 & 16777215

    r3 = r3 * 65899

    # keep 24 bits
    r3 = r3 & 16777215

    if 256 > r4:
      break
    r4 = r4 // 256

  # print(r, r4, len(values))
  if r3 in values:
    print('Solution 2: ', values[-1])
    break
  values.append(r3)
