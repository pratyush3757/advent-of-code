import os.path

with open(os.path.join(
    os.path.split(os.path.dirname(__file__))[0], 
    'input', 
    'advent_2015_day_07.txt')) as f:
    lines = f.readlines()

# part1
known_wires = dict()
unknown_wires = dict()
mask = 2 ** 16 - 1

def andop(x, y):
    return x & y

def orop(x, y):
    return x | y

def xorop(x, y):
    return x & y

def notop(x):
    return ~x

def bitleft(x, y):
    return (x << y) & mask

def bitright(x, y):
    return (x >> y) & mask

def resolvable(x):
    return (x in known_wires) or (x.isnumeric())

def get_val(x):
    if x.isnumeric():
        return int(x)
    else:
        return known_wires[x]
    
def split_incoming(incoming: str) -> tuple[list[str], str]:
    match incoming.split(' '):
        case [wire]:
            return ([wire], "")
        case [OP, wire]:
            return ([wire], OP)
        case [wire_1, OP, wire_2]:
            return ([wire_1, wire_2], OP)

def solve(operands, operator):
    operands = list(map(get_val, operands))
    if (len(operands) == 1) and (operator == ""):
        return operands[0]
    match operator:
        case "NOT":
            return notop(*operands)
        case "AND":
            return andop(*operands)
        case "OR":
            return orop(*operands)
        case "XOR":
            return xorop(*operands)
        case "LSHIFT":
            return bitleft(*operands)
        case "RSHIFT":
            return bitright(*operands)

def populate_wire_dicts():
    global known_wires, unknown_wires
    known_wires = dict()
    unknown_wires = dict()
    
    for line in lines:
        incoming, outgoing = list(map(str.strip, line.split("->")))
        operands, operator = split_incoming(incoming)
        
        if all(resolvable(op) for op in operands):
            # print(incoming, outgoing)
            known_wires[outgoing] = solve(operands, operator)
        else:
            unknown_wires[outgoing] = incoming

def resolve_unknown_wires():
    while(len(unknown_wires) != 0):
        if 'a' in known_wires:
            print(len(unknown_wires))
            break
        removable_keys = []
        for key, value in unknown_wires.items():
            operands, operator = split_incoming(value)
            if all(resolvable(op) for op in operands):
                known_wires[key] = solve(operands, operator)
                # print(value, key, known_wires[key])
                removable_keys.append(key)
        for key in removable_keys:
            unknown_wires.pop(key, None)

def part_1():
    populate_wire_dicts()
    resolve_unknown_wires()
    print(known_wires['a'])

part_1()

# part2

def part_2():
    # assuming part_1 has been called
    wire_b = known_wires['a']
    populate_wire_dicts()
    known_wires['b'] = wire_b
    resolve_unknown_wires()
    print(known_wires['a'])

part_2()
