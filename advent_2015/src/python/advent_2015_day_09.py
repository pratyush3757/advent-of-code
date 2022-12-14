import re
with open('day9_input.txt') as f:
    lines = f.readlines()

# part1

distance_table = dict()
complete_routes = []

def split_line(line: str):
    cities, distance = line.split(" = ")
    cityA, cityB = cities.split(" to ")
    return cityA, cityB, int(distance)

def new_if_absent(city):
    if city in distance_table:
        return
    else:
        #distance_table[city] = {city: 0}
        distance_table[city] = dict()

def record_distance(cityA, cityB, distance):
    new_if_absent(cityA)
    new_if_absent(cityB)
    distance_table[cityA][cityB] = distance
    distance_table[cityB][cityA] = distance

def min_route_length(current_route: tuple[list[str], int]) -> int:
    all_cities = distance_table.keys()
    next_candidates = set(all_cities).difference(set(current_route[0]))
    if len(next_candidates) == 0:
        complete_routes.append(current_route)
        return current_route[1]
    
    routes = []
    for i in next_candidates:
        next_jump = distance_table.get(current_route[0][-1]).get(i)
        routes.append((current_route[0] + [i], current_route[1] + next_jump))
    
    return min(map(min_route_length, routes))

def part1():
    for line in lines:
        line = line.strip()
        cityA, cityB, distance = split_line(line)
        record_distance(cityA, cityB, distance)
        # print(cityA, cityB, distance)
        
    # print(distance_table)
    
    all_cities = distance_table.keys()
    route_lengths = list(map(lambda x: min_route_length(([x], 0)), all_cities))
    shortest_route_length = min(route_lengths)
    print(shortest_route_length)
    
    for i in complete_routes:
        if i[1] == shortest_route_length:
            print(i)
    
part1()

# part2
def max_route_length(current_route: tuple[list[str], int]) -> int:
    all_cities = distance_table.keys()
    next_candidates = set(all_cities).difference(set(current_route[0]))
    if len(next_candidates) == 0:
        return current_route[1]
    
    routes = []
    for i in next_candidates:
        next_jump = distance_table.get(current_route[0][-1]).get(i)
        routes.append((current_route[0] + [i], current_route[1] + next_jump))
    
    return max(map(max_route_length, routes))

def part2():
     
    all_cities = distance_table.keys()
    route_lengths = list(map(lambda x: max_route_length(([x], 0)), all_cities))
    longest_route_length = max(route_lengths)
    print(longest_route_length)
    
    for i in complete_routes:
        if i[1] == longest_route_length:
            print(i)

part2()

print(len(complete_routes))
