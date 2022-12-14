input_str = "1113222113"

# part1
def lookandsay(input_sequence: str) -> str:
    output_str = ""
    run_char = input_sequence[0]
    run_length = 1
    for i in input_sequence[1:]:
        if i == run_char:
            run_length += 1
        else:
            output_str += f"{run_length}{run_char}"
            run_length = 1
            run_char = i
    output_str += f"{run_length}{run_char}"
    return output_str

def part1():
    print(input_str)
    output_str = input_str
    for i in range(40):
        output_str = lookandsay(output_str)
        # print(output_str)
    print(len(output_str))
    
part1()

# part2
def part2():
    print(input_str)
    output_str = input_str
    for i in range(50):
        output_str = lookandsay(output_str)
        # print(output_str)
    print(len(output_str))
    
part2()
