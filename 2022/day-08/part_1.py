def main():
    input = open(0).read()
    grid = [[int(x) for x in line] for line in input.splitlines()]
    
    total = 0
    for (r,row) in enumerate(grid):
        for (c,ch) in enumerate(row):
            col = [x[c] for x in grid]
            left,right = row[:c],row[c+1:]
            up,down = col[:r],col[r+1:]
            ml = max(left) if left else -1
            mr = max(right) if right else -1
            mu = max(up) if up else -1
            md = max(down) if down else -1

            if ch>ml or ch>mr or ch>mu or ch>md:
                total +=1

    return total
    


if __name__ == '__main__':
    print(main())
