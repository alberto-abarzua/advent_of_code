def get_file(path, fs):
    cur = fs
    for p in path:
        cur = cur[p]

    return cur


def get_size(path, fs):

    wd = get_file(path, fs)
    if "___size" in wd:
        return wd["___size"]
    total = 0
    for (file, value) in wd.items():
        if isinstance(value, int):
            total += value
        else:
            total += get_size(path+[file], fs)

    wd["___size"] = total
    return total


def get_dirs(path, fs):
    dirs = []
    for (file, value) in fs.items():
        if isinstance(value, int):
            continue
        dirs.append((path+[file], file))
        dirs += get_dirs(path+[file], fs[file])
    return dirs


def main():
    input = open(0).read()
    lines = input.splitlines()
    commands = []
    for line in lines:
        if line.startswith("$"):
            commands.append((line.removeprefix("$ "), []))
            continue
        commands[-1][1].append(line)

    fs = {"/": {}}
    wds = ["/"]
    commands = commands[1:]
    for (com, output) in commands:

        if 'ls' in com:
            wd = get_file(wds, fs)
            l = [val.split() for val in output]
            for (size, name) in l:
                if size == "dir":
                    if name not in wd:
                        wd[name] = {}
                else:
                    wd[name] = int(size)

        if 'cd' in com:
            arg = com.split()[1]

            if arg == "..":
                wds.pop()
                continue
            wd = get_file(wds, fs)
            if arg not in wd:
                wd[arg] = {}
            wds.append(arg)

    sizes = [get_size(d, fs) for (d, _) in get_dirs([], fs)]
    sizes.sort()
    total = get_size(["/"], fs)
    free = 70000000 - total
    for size in sizes:
        if size + free >= 30000000:
            return size


if __name__ == '__main__':
    print(main())
