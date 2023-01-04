import os.path

with open(os.path.join(
    os.path.split(os.path.dirname(__file__))[0], 
    'input', 
    'advent_2015_day_01.txt')) as f:
    lines = f.readlines()

input_str = lines[0]

# part1
print(input_str.count('(') - input_str.count(')'))

# part2
rolling_sum = 0
for pos, i in enumerate(input_str, 1):
    rolling_sum += 1 if i == '(' else -1
    if rolling_sum == -1:
        print(pos)
        break
