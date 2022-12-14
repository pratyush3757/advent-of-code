import re

with open('day5_input.txt') as f:
    lines = f.readlines()

# part1
sum_nice_lines = 0
pattern_vowels = re.compile(r"[aeiou]")
pattern_consecutive_letters = re.compile(r"(.)\1")
pattern_forbidden = re.compile(r"(ab)|(cd)|(pq)|(xy)")

for line in lines:
    res_forbidden = re.findall(pattern_forbidden, line)
    if len(res_forbidden) != 0:
        continue

    res_vowel = re.findall(pattern_vowels, line)
    if len(res_vowel) < 3:
        continue
    
    res_consecutive = re.findall(pattern_consecutive_letters, line)
    if len(res_consecutive) < 1:
        continue

    # print(line[:-1])
    sum_nice_lines += 1

print(sum_nice_lines)

# part2
sum_nice_lines = 0
pattern_repeating_twice = re.compile(r"((\w\w))(?=.*\1)")
pattern_letter_between_same = re.compile(r"(\w).(\1)")

for line in lines:
    res_repeating_twice = re.findall(pattern_repeating_twice, line)
    if len(res_repeating_twice) == 0:
        continue

    res_letter_between_same = re.findall(pattern_letter_between_same, line)
    if len(res_letter_between_same) == 0:
        continue
    
    # print(line[:-1])
    sum_nice_lines += 1

print(sum_nice_lines)
