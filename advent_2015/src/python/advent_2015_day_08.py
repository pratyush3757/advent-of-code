import re
with open('day8_input.txt') as f:
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
        # print(line, len(line))
        
        # line_2 = line
        # escaped_line2 = line_2.encode().decode('unicode-escape')
        # print('old', escaped_line2, len(escaped_line2)-2)
        escaped_line = shorten(line)
        # print('new', escaped_line, len(escaped_line))
        
        sum_escaped += len(escaped_line)
        
    print(sum_raw, sum_escaped, sum_raw-sum_escaped)
    
part1()

# part2

def expand(line: str):
    # line = re.sub(r'\\\\', r'\\\\\\\\', line)
    line = re.sub(r'\\', r'\\\\', line)
    # line = re.sub(r'\\x..', r'\\\\x..', line)
    line = re.sub(r'"', r'\"', line)
    return line

def part2():
    sum_raw = 0
    sum_expanded = 0
    for line in lines:
        line = line.strip()
        sum_raw += len(line)
        # print(line, len(line))
        expanded_line = '"' + expand(line) + '"'
        # print(expanded_line, len(expanded_line))
        
        sum_expanded += len(expanded_line)
        
    print(sum_raw, sum_expanded, sum_expanded-sum_raw)

part2()
