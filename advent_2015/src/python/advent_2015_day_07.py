with open('day7_input.txt') as f:
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
    
def split_incoming(incoming):
    incoming_op = incoming.split(" ")
    operands = []
    operator = ""
    if (len(incoming_op) == 1):
        operands.append(incoming_op[0])
    elif (len(incoming_op) == 2):
        operator = incoming_op[0]
        operands.append(incoming_op[1])
    else:
        operator = incoming_op[1]
        operands.extend([incoming_op[0], incoming_op[2]])
        
    return (operands, operator)

def solve(operands, operator):
    if (len(operands) == 1) and (operator == ""):
        return get_val(operands[0])
    elif (operator == "NOT"):
        return notop(get_val(operands[0]))
    elif (operator == "AND"):
        return andop(get_val(operands[0]), get_val(operands[1]))
    elif (operator == "OR"):
        return orop(get_val(operands[0]), get_val(operands[1]))
    elif (operator == "XOR"):
        return xorop(get_val(operands[0]), get_val(operands[1]))
    elif (operator == "LSHIFT"):
        return bitleft(get_val(operands[0]), get_val(operands[1]))
    elif (operator == "RSHIFT"):
        return bitright(get_val(operands[0]), get_val(operands[1]))

for line in lines:
    incoming, outgoing = list(map(str.strip, line.split("->")))
    operands, operator = split_incoming(incoming)
    
    if all(resolvable(op) for op in operands):
        print(incoming, outgoing)
        known_wires[outgoing] = solve(operands, operator)
    else:
        unknown_wires[outgoing] = incoming

while(len(unknown_wires) != 0):
    if 'a' in known_wires:
        print(len(unknown_wires))
        break
    removable_keys = []
    for key, value in unknown_wires.items():
        operands, operator = split_incoming(value)
        if all(resolvable(op) for op in operands):
            known_wires[key] = solve(operands, operator)
            print(value, key, known_wires[key])
            removable_keys.append(key)
    for key in removable_keys:
        unknown_wires.pop(key, None)

print(known_wires)


# part2
known_wires = dict()
unknown_wires = dict()

for line in lines:
    incoming, outgoing = list(map(str.strip, line.split("->")))
    operands, operator = split_incoming(incoming)
    
    if all(resolvable(op) for op in operands):
        print(incoming, outgoing)
        known_wires[outgoing] = solve(operands, operator)
    else:
        unknown_wires[outgoing] = incoming
        
known_wires['b'] = 46065

while(len(unknown_wires) != 0):
    if 'a' in known_wires:
        print(len(unknown_wires))
        break
    removable_keys = []
    for key, value in unknown_wires.items():
        operands, operator = split_incoming(value)
        if all(resolvable(op) for op in operands):
            known_wires[key] = solve(operands, operator)
            print(value, key, known_wires[key])
            removable_keys.append(key)
    for key in removable_keys:
        unknown_wires.pop(key, None)

print(known_wires)
