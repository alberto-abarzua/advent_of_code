def main():
    input = open(0).read()
    vals = [[[int(x) for x in r.split("-")] for r in p.split(",")] for p in input.splitlines()]
    count = 0
    for (a, b), (x, y) in vals:
        first = range(a, b + 1)
        second = range(x, y + 1)
        if  a in second or b in second or x in first or y in first:
            count += 1
    return count

if __name__ == '__main__':
    print(main())
