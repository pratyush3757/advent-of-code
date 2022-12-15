import re
import os.path
from itertools import permutations

with open(os.path.join(
    os.path.split(os.path.dirname(__file__))[0], 
    'input', 
    'advent_2015_day_09.txt')) as f:
    lines = f.readlines()

distance_table = dict()

def split_line(line: str):
    cities, distance = line.split(" = ")
    cityA, cityB = cities.split(" to ")
    return cityA, cityB, int(distance)

def new_if_absent(city):
    if city in distance_table:
        return
    else:
        distance_table[city] = dict()

def record_distance(cityA, cityB, distance):
    new_if_absent(cityA)
    new_if_absent(cityB)
    distance_table[cityA][cityB] = distance
    distance_table[cityB][cityA] = distance

def calculate_route_length(route: list[str]):
    route_length = 0
    for i, city in enumerate(route[1:], 1):
        route_length += distance_table.get(route[i-1]).get(city)
    return route_length

# part1
def part1():
    for line in lines:
        line = line.strip()
        cityA, cityB, distance = split_line(line)
        record_distance(cityA, cityB, distance)
    
    all_cities = distance_table.keys()
    route_permutations = list(permutations(all_cities))
    shortest_route_length = min(map(calculate_route_length, route_permutations))
    print(shortest_route_length)
    
    for i in route_permutations:
        if i[1] == shortest_route_length:
            print(i)
    
part1()

# part2
def part2():
     
    all_cities = distance_table.keys()
    route_permutations = list(permutations(all_cities))
    longest_route_length = max(map(calculate_route_length, route_permutations))
    print(shortest_route_length)
    
    for i in route_permutations:
        if i[1] == shortest_route_length:
            print(i)
    
    print(len(route_permutations))

part2()

