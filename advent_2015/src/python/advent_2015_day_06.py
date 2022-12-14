with open('day6_input.txt') as f:
    lines = f.readlines()

# part1

lights = [[False for _ in range(1000)] for _ in range(1000)]

def turn_on(ul: tuple[int, int], lr: tuple[int, int]):
    global lights
    ulx, uly = ul
    lrx, lry = lr
    for i in range(ulx, lrx + 1):
        for j in range(uly, lry + 1):
            lights[i][j] = True

def turn_off(ul: tuple[int, int], lr: tuple[int, int]):
    global lights
    ulx, uly = ul
    lrx, lry = lr
    for i in range(ulx, lrx + 1):
        for j in range(uly, lry + 1):
            lights[i][j] = False

def toggle(ul: tuple[int, int], lr: tuple[int, int]):
    global lights
    ulx, uly = ul
    lrx, lry = lr
    for i in range(ulx, lrx + 1):
        for j in range(uly, lry + 1):
            lights[i][j] = not lights[i][j]

for line in lines:
    command = line.rsplit(' ', 3)
    # print(command)
    
    x, y = command[1].split(',')
    ul = (int(x), int(y))

    x, y = command[3].split(',')
    lr = (int(x), int(y))
    # print(ul, lr)
    
    if command[0] == "turn on":
        turn_on(ul, lr)
    elif command[0] == "turn off":
        turn_off(ul, lr)
    elif command[0] == "toggle":
        toggle(ul, lr)

print(sum(sum(row) for row in lights))

# part2

lights = [[0 for _ in range(1000)] for _ in range(1000)]

def turn_on_bright(ul: tuple[int, int], lr: tuple[int, int]):
    global lights
    ulx, uly = ul
    lrx, lry = lr
    for i in range(ulx, lrx + 1):
        for j in range(uly, lry + 1):
            lights[i][j] += 1

def turn_off_dull(ul: tuple[int, int], lr: tuple[int, int]):
    global lights
    ulx, uly = ul
    lrx, lry = lr
    for i in range(ulx, lrx + 1):
        for j in range(uly, lry + 1):
            lights[i][j] -= 1
            if lights[i][j] < 0:
                lights[i][j] = 0

def toggle_more_bright(ul: tuple[int, int], lr: tuple[int, int]):
    global lights
    ulx, uly = ul
    lrx, lry = lr
    for i in range(ulx, lrx + 1):
        for j in range(uly, lry + 1):
            lights[i][j] += 2


for line in lines:
    command = line.rsplit(' ', 3)
    # print(command)
    
    x, y = command[1].split(',')
    ul = (int(x), int(y))

    x, y = command[3].split(',')
    lr = (int(x), int(y))
    # print(ul, lr)
    
    if command[0] == "turn on":
        turn_on_bright(ul, lr)
    elif command[0] == "turn off":
        turn_off_dull(ul, lr)
    elif command[0] == "toggle":
        toggle_more_bright(ul, lr)

print(sum(sum(row) for row in lights))
