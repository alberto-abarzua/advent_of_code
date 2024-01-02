

def main():
    input = open(0).read()
    blocks = [[int(x)  for x in block.split("\n") if len(x)>0] for block in input.split("\n\n")]
    m = 0
    for b in blocks:
        m = max(m,sum(b))
    return m
    


if __name__ == '__main__':
    print(main())
