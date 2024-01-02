def main():
    input = open(0).read()
    total =0

    for line in input.splitlines():
        cur = {}
        start = 0
        for (i,ch) in enumerate(line):
            if i-start == 14:
                total += start+14
                break
            if ch in cur and start <= cur[ch]:
                start = cur[ch] + 1


            cur[ch] = i

    return total

if __name__ == '__main__':
    print(main())
