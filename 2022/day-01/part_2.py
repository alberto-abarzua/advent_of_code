def main():
    input = open(0).read()
    blocks = [[int(x)  for x in block.split("\n") if len(x)>0] for block in input.split("\n\n")]
    total = []
    for b in blocks:
        total.append(sum(b))
    total.sort(reverse=True)
    
    return total[0] + total[1] + total[2]
    


if __name__ == '__main__':
    print(main())
