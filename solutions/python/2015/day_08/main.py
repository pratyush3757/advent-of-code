import re
import os.path

with open(os.path.join(
    os.path.split(os.path.dirname(__file__))[0], 
    'input', 
    'advent_2015_day_08.txt')) as f:
    lines = f.readlines()

# part1
def shorten(line: str):
    line = re.sub(r'\\\\', r'.', line)
    line = re.sub(r'\\"', r'.', line)
    line = re.sub(r'\\x..', r'.', line)
    line = re.sub(r'"', r'', line)
    return line

def part1():
    sum_raw = 0
    sum_escaped = 0
    for line in lines:
        line = line.strip()
        sum_raw += len(line)
        
        escaped_line = shorten(line)
        sum_escaped += len(escaped_line)
        
    print(sum_raw, sum_escaped, sum_raw-sum_escaped)
    
part1()

# part2
def expand(line: str):
    line = re.sub(r'\\', r'\\\\', line)
    line = re.sub(r'"', r'\"', line)
    return line

def part2():
    sum_raw = 0
    sum_expanded = 0
    for line in lines:
        line = line.strip()
        sum_raw += len(line)
        
        expanded_line = '"' + expand(line) + '"'
        sum_expanded += len(expanded_line)
        
    print(sum_raw, sum_expanded, sum_expanded-sum_raw)

part2()
