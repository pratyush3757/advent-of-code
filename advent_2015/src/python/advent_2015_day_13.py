import re
with open('day13_input.txt') as f:
    lines = f.readlines()

# part1

happiness_table = dict()
complete_plans = []

def split_line(line: str):
    line = line.rstrip(".")
    firstpart, personB = line.split(" happiness units by sitting next to ")
    personA, points = firstpart.split(" would ")
    sign, points = points.split(" ")
    points = int(points)
    points = points if sign == 'gain' else -points
    return personA, personB, points

def new_if_absent(person):
    if person in happiness_table:
        return
    else:
        happiness_table[person] = dict()

def record_happiness(personA, personB, happiness):
    new_if_absent(personA)
    happiness_table[personA][personB] = happiness

def max_plan_length(current_plan: tuple[list[str], int]) -> int:
    all_people = happiness_table.keys()
    next_candidates = set(all_people).difference(set(current_plan[0]))
    if len(next_candidates) == 0:
        # print(current_plan)
        happiness = current_plan[1]
        happiness += happiness_table.get(current_plan[0][-1]).get(current_plan[0][0])
        happiness += happiness_table.get(current_plan[0][0]).get(current_plan[0][-1])
        complete_plans.append((current_plan[0], happiness))
        return happiness
    
    plans = []
    for i in next_candidates:
        next_jump = happiness_table.get(current_plan[0][-1]).get(i)
        next_jump += happiness_table.get(i).get(current_plan[0][-1])
        plans.append((current_plan[0] + [i], current_plan[1] + next_jump))
    
    return max(map(max_plan_length, plans))

def part1():
    for line in lines:
        line = line.strip()
        personA, personB, happiness = split_line(line)
        record_happiness(personA, personB, happiness)
        # print(personA, personB, happiness)
        
    # print(happiness_table)
    
    all_people = happiness_table.keys()
    plan_lengths = list(map(lambda x: max_plan_length(([x], 0)), all_people))
    longest_plan_length = max(plan_lengths)
    print(plan_lengths, longest_plan_length)
    
    for i in complete_plans:
        if i[1] == longest_plan_length:
            print(i)
    
part1()
print(len(complete_plans))

# part2
complete_plans = []
def part2():
    all_people = happiness_table.keys()
    new_if_absent("You")
    for person in all_people:
        happiness_table[person]["You"] = 0
        happiness_table["You"][person] = 0

    plan_lengths = list(map(lambda x: max_plan_length(([x], 0)), all_people))
    longest_plan_length = max(plan_lengths)
    print(plan_lengths, longest_plan_length)
    
    for i in complete_plans:
        if i[1] == longest_plan_length:
            print(i)

part2()

print(len(complete_plans))
