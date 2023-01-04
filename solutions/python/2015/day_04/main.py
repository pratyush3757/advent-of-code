import hashlib
input_str = "yzbqklnj"

def hash_till(starts_with_prefix: str) -> str:
    suffix = 1
    while(True):
        output_hash = hashlib.md5((input_str + str(suffix)).encode('utf-8')).hexdigest()
        if output_hash.startswith(starts_with_prefix):
            return f"hash: {output_hash}, suffix: {suffix}"
        suffix+=1

# part1
print(hash_till("00000"))

# part2
print(hash_till("000000"))
