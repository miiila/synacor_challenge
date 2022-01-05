from itertools import permutations

for p in permutations([2,7,3,5,9]):
    res = p[0]+p[1]*p[2]**2+p[3]**3-p[4]
    if res == 399:
        print(p)


