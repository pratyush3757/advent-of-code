with open('day1_input.txt') as f:
    lines = f.readlines()

input_str = lines[0]

# part1
print(input_str.count('(') - input_str.count(')'))

# part2
sum_1 = 0
for pos, i in enumerate(input_str, 1):
    if i == '(':
        sum_1 +=1
    else:
        sum_1 -=1
    if sum_1 == -1:
        print(pos)
        break
