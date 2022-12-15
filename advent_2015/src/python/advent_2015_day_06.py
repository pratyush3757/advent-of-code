import os.path

with open(os.path.join(
    os.path.split(os.path.dirname(__file__))[0], 
    'input', 
    'advent_2015_day_06.txt')) as f:
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

def convert_to_coord(ul: str, lr: str):
    ul = tuple(map(int, ul.split(",")))
    lr = tuple(map(int, lr.split(",")))
    return ul, lr

def part_1():
    for line in lines:
        match line.split(' '):
            case ["turn", "on", ul, "through", lr]:
                ul, lr = convert_to_coord(ul, lr)
                turn_on(ul, lr)
            case ["turn", "off", ul, "through", lr]:
                ul, lr = convert_to_coord(ul, lr)
                turn_off(ul, lr)
            case ["toggle", ul, "through", lr]:
                ul, lr = convert_to_coord(ul, lr)
                toggle(ul, lr)

    print(sum(sum(row) for row in lights))

part_1()

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

def part_2():        
    for line in lines:
        match line.split(' '):
            case ["turn", "on", ul, "through", lr]:
                ul, lr = convert_to_coord(ul, lr)
                turn_on_bright(ul, lr)
            case ["turn", "off", ul, "through", lr]:
                ul, lr = convert_to_coord(ul, lr)
                turn_off_dull(ul, lr)
            case ["toggle", ul, "through", lr]:
                ul, lr = convert_to_coord(ul, lr)
                toggle_more_bright(ul, lr)

    print(sum(sum(row) for row in lights))

part_2()
