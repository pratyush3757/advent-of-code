import os.path

with open(os.path.join(
    os.path.split(os.path.dirname(__file__))[0], 
    'input', 
    'advent_2015_day_02.txt')) as f:
    lines = f.readlines()

# part 1
sum_area = 0
sum_ribbon = 0
for line in lines:
    line_list = list(map(int, line.split('x', 2)))
    l, w, h = line_list
    
    largest_dimension = max(l,w,h)
    
    wrap_area = 2*l*w + 2*l*h + 2*w*h
    extra_area = (l*w*h)/largest_dimension
    total_area = wrap_area + extra_area
    sum_area += total_area
    
    # part 2
    smallest_face_perimeter = 2*(l+w+h) - 2*(largest_dimension)
    volume = l*w*h
    total_ribbon_length = smallest_face_perimeter + volume
    sum_ribbon += total_ribbon_length

print(sum_area, sum_ribbon)
