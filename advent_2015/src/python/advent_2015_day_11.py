import re
input_str = "hxbxwxba"

# part1
def increment(input_sequence: str) -> str:
    if len(input_sequence) == 1:
        i = ord(input_sequence)
        return chr(i+1) if i < 122 else 'a'
    
    next_char = increment(input_sequence[-1])
    if next_char == 'a':
        output_str = increment(input_sequence[:-1]) + 'a'
    else:
        output_str = input_sequence[:-1] + next_char

    return output_str

def valid_pass(input_sequence: str) -> bool:
    consecutive_run = False
    for i in range(1, len(input_sequence) - 1):
        last_char = ord(input_sequence[i-1])
        curr_char = ord(input_sequence[i])
        next_char = ord(input_sequence[i+1])
        if (curr_char == last_char + 1) and (curr_char == next_char - 1):
            consecutive_run = True
    
    forbidden_letters_absent = len(re.findall(r"[iol]", input_sequence)) == 0
    
    double_pattern = len(re.findall(r"(.)\1", input_sequence)) > 1
    
    return consecutive_run and forbidden_letters_absent and double_pattern

def part1():
    output_str = input_str
    while(not valid_pass(output_str)):
        output_str = increment(output_str)
        
    print(output_str)
    
part1()

# part2
part2_input_str = "hxbxxyzz"
def part2():
    output_str = increment(part2_input_str)
    while(not valid_pass(output_str)):
        output_str = increment(output_str)
        
    print(output_str)
    
part2()
