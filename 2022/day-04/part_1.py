def main():
    input = open(0).read()
    vals = [[[int(x) for x in r.split("-")] for r in p.split(",")] for p in input.splitlines()]
    count = 0
    for (a,b),(x,y) in vals:
        if ((a <= x and y<=b) or (x<=a and b<=y) ):
            count += 1
    return count
    
if __name__ == '__main__':
    print(main())
