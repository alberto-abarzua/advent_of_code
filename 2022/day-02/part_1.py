

def main():
    input = open(0).read()
    m = {
        "A": 1, # rock
        "B": 2, # paper
        "C": 3, # sicssors
        "X": 1, # rock
        "Y": 2, # paper
        "Z": 3  # sicssors
    }

    vals = [[m[ch] for ch in x.split(" ")] for x in input.splitlines()]
    total = 0
    for (a,b) in vals:
        if a == b:  # draw
            total += 3 + b
            continue
        if a == 1: # player a rock
            if b == 2: # paper - win
                total += b + 6
            if b == 3: # sicssors - lose
                total += b
        if a == 2: # player a paper
            if b == 1: # rock - lose
                total += b
            if b == 3: # sicssors - win
                total += b + 6
        if a == 3: # player a sicssors
            if b == 1: # rock - win
                total += b + 6
            if b == 2: # paper - lose
                total += b
    return total


if __name__ == '__main__':
    print(main())
