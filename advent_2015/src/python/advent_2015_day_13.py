import re
import os.path
from itertools import permutations

with open(os.path.join(
    os.path.split(os.path.dirname(__file__))[0], 
    'input', 
    'advent_2015_day_13.txt')) as f:
    lines = f.readlines()

happiness_table = dict()

def split_line(line: str):
    match line.split(' '):
        case [personA, "would", sign, points,"happiness", "units", "by", 'sitting', "next", "to", personB]:
            points = int(points)
            points = points if sign == 'gain' else -points
            return personA, personB.rstrip('.'), points

def new_if_absent(person):
    if person in happiness_table:
        return
    else:
        happiness_table[person] = dict()

def record_happiness(personA, personB, happiness):
    new_if_absent(personA)
    happiness_table[personA][personB] = happiness

def calculate_happiness(arrangement: list[str]):
    total_happiness = 0
    for i, person in enumerate(arrangement):
        total_happiness += happiness_table.get(person).get(arrangement[i-1])
        total_happiness += happiness_table.get(arrangement[i-1]).get(person)
    return total_happiness

# part1
def part_1():
    for line in lines:
        line = line.strip()
        personA, personB, happiness = split_line(line)
        record_happiness(personA, personB, happiness)
    
    all_people = happiness_table.keys()
    arrangement_permutations = list(permutations(all_people))
    max_happiness = max(map(calculate_happiness, arrangement_permutations))
    print(max_happiness)
    
    for i in arrangement_permutations:
        if i[1] == max_happiness:
            print(i)
    
    print(len(arrangement_permutations), "permutations")
    
part_1()

# part2
def part_2():
    all_people = happiness_table.keys()
    new_if_absent("You")
    for person in all_people:
        happiness_table[person]["You"] = 0
        happiness_table["You"][person] = 0

    arrangement_permutations = list(permutations(all_people))
    max_happiness = max(map(calculate_happiness, arrangement_permutations))
    print(max_happiness)
    
    for i in arrangement_permutations:
        if i[1] == max_happiness:
            print(i)
    
    print(len(arrangement_permutations), "permutations")

part_2()
