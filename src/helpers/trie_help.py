import csv

ops = {}

with open('6502ops.csv', newline='') as csvfile:
    reader = csv.DictReader(csvfile)
    for row in reader:
        ops[row['opcode']] = [
            row['mnemonic'],
            row['addressing mode'],
            row['bytes'],
            row['cycles'],
            row['flags'],
        ]

sorted_dict = {k: ops[k] for k in sorted(ops)}

for key, value in sorted_dict.items():
    print(f'{key}: {value}')

with open('output.txt', 'w') as file:
    for op in ops:
        line = f"dis_trie.insert(&{op}_u8, \"{",".join(ops[op])}\".to_string());"
        file.write(line + '\n')