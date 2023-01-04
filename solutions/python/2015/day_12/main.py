import re
import os.path

with open(os.path.join(
    os.path.split(os.path.dirname(__file__))[0], 
    'input', 
    'advent_2015_day_12.txt')) as f:
    lines = f.readlines()

# part1
def to_num(input_list):
    return list(map(int, input_list))

def part1():
    total = 0
    for line in lines:
        string_numerals = re.findall(r"[-+]?\d+", line)
        int_numerals = to_num(string_numerals)
        # print(int_numerals)
        total += sum(int_numerals)
    print(total)
    
part1()
