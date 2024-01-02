

def main():
    input = open(0).read()
    sacks = [ (x[:len(x)//2], x[len(x)//2:]) for x in input.splitlines() ]

    repeated = []
    for (a,b) in sacks:
        l = set(a) & set(b)
        repeated.extend(list(l))

    sum =0
    for ch in repeated:
        if ord(ch) > 96:
            sum += ord(ch) - 96
        else:
            sum += ord(ch) - 38
    return sum


    
    


if __name__ == '__main__':
    print(main())
