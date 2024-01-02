def find_first(seq,t):
    for (i,x) in enumerate(seq):
        if x>=t:
            return i+1
    return len(seq)

def main():
    input = open(0).read()
    grid = [[int(x) for x in line] for line in input.splitlines()]
    
    m=0

    for (r,row) in enumerate(grid):
        for (c,ch) in enumerate(row):
            col = [x[c] for x in grid]
            left,right = row[:c],row[c+1:]
            left.reverse()
            up,down = col[:r],col[r+1:]
            up.reverse()
            fl = find_first(left,ch)
            fr = find_first(right,ch)
            fu = find_first(up,ch)
            fd = find_first(down,ch)

            m = max(m, fl*fr*fu*fd)

            

    return m
    


if __name__ == '__main__':
    print(main())
