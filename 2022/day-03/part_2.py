def main():
    input = open(0).read()
    sacks = [ (x[:len(x)//2], x[len(x)//2:]) for x in input.splitlines() ]

    vals = []
    cur = []
    for (i,(a,b)) in enumerate(sacks):
        if i % 3 == 0 and cur:
            badge = set.intersection(*cur)

            vals.append(badge.pop())
            cur = []
        cur.append(set(a) | set(b))

    vals.append(set.intersection(*cur).pop())
    sum =0
    for ch in vals:
        if ord(ch) > 96:
            sum += ord(ch) - 96
        else:
            sum += ord(ch) - 38
    return sum


    
    


if __name__ == '__main__':
    print(main())
