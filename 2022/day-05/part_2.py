def main():
    input = open(0).read()
    lines = input.splitlines()
    start = 0
    boxes = []
    for i, line in enumerate(lines):
        if line.startswith(' 1'):
            start = i
            break
        cur = []
        for x in line.split():
            x = x.replace("[", "").replace("]", "")
            place = line.index(x)
            line = line[:place] + "-" + line[place + len(x):]
            place = place // 4
            cur.append((x, place))
        boxes.append(cur)

    boxes.reverse()
    l = max([len(row) for row in boxes])
    stacks = [[] for _ in range(l)]
    for row in boxes:
        for (x, place) in row:
            stacks[place].append(x)

    rest = lines[start+2:]
    for line in rest:
        line = line.replace("move", "").replace("to", "").replace("from", "")
        amount, fr, to = [int(x) for x in line.split()]
        fr -= 1
        to -= 1
        stacks[to].extend(stacks[fr][-amount:])
        stacks[fr] = stacks[fr][:-amount]

    return "".join([s[-1] for s in stacks])


if __name__ == '__main__':
    print(main())
