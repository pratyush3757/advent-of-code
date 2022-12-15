import os.path

with open(os.path.join(
    os.path.split(os.path.dirname(__file__))[0], 
    'input', 
    'advent_2015_day_03.txt')) as f:
    lines = f.readlines()
    
input_str = lines[0]

# part1
house_set = set()
pos_x = 0
pos_y = 0

for direction in input_str:
    match direction:
        case "^":
            pos_y += 1
        case "v":
            pos_y -= 1
        case ">":
            pos_x += 1
        case "<":
            pos_x-=1
        
    house_set.add((pos_x,pos_y))

print(len(house_set))

# part2
house_set_2 = set()
pos_santa_x = 0
pos_santa_y = 0
pos_robo_x = 0
pos_robo_y = 0

for turn, direction in enumerate(input_str):
    increment_x = increment_y = 0
    match direction:
        case "^":
            increment_y += 1
        case "v":
            increment_y -= 1
        case ">":
            increment_x += 1
        case "<":
            increment_x -= 1
    
    if turn % 2 == 0:
        pos_santa_x += increment_x
        pos_santa_y += increment_y
        house_set_2.add((pos_santa_x,pos_santa_y))
    else:
        pos_robo_x += increment_x
        pos_robo_y += increment_y
        house_set_2.add((pos_robo_x, pos_robo_y))

print(len(house_set_2))
