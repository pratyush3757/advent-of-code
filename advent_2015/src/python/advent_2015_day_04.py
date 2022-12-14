import hashlib
input_str = "yzbqklnj"

# part1
suffix = 1
while(True):
    output_hash = hashlib.md5((input_str + str(suffix)).encode('utf-8')).hexdigest()
    if output_hash.startswith('00000'):
        print(output_hash)
        print(suffix)
        break
    
    suffix+=1

# part2
suffix = 1
while(True):
    output_hash = hashlib.md5((input_str + str(suffix)).encode('utf-8')).hexdigest()
    if output_hash.startswith('000000'):
        print(output_hash)
        print(suffix)
        break
    
    suffix+=1
