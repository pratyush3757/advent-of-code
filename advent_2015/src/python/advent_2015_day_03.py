with open('day3_input.txt') as f:
    lines = f.readlines()

input_str = lines[0]

# part1
house_set = set()
pos_x = 0
pos_y = 0

for direction in input_str:
    if direction == "^":
        pos_y+=1
    elif direction == "v":
        pos_y-=1
    elif direction == ">":
        pos_x+=1
    elif direction == "<":
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
    if direction == "^":
        increment_y += 1
    elif direction == "v":
        increment_y -= 1
    elif direction == ">":
        increment_x += 1
    elif direction == "<":
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
