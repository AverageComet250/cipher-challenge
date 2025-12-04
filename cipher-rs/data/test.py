import os
import re

def lowercase_keys(path):
    with open(path, "r") as f:
        lines = f.read().splitlines()

    if len(lines) <= 2:
        return

    header = lines[0]
    footer = lines[-1]
    body = lines[1:-1]

    # Regex to match "KEY" => VALUE,
    pattern = re.compile(r'^"(.*)"\s*=>\s*(.*),$')

    new_body = []
    for line in body:
        m = pattern.match(line)
        if m:
            key, value = m.groups()
            key_lower = key.lower()
            new_body.append(f'"{key_lower}" => {value},')
        else:
            # keep line as-is if it doesn't match
            new_body.append(line)

    with open(path, "w") as f:
        f.write(header + "\n")
        for line in new_body:
            f.write(line + "\n")
        f.write(footer + "\n")


# Apply to all .txt files in current directory
for filename in os.listdir("."):
    if filename in ["1gram.txt", "2gram.txt", "3gram.txt", "4gram.txt", "words.txt"]:
        print("Processing:", filename)
        lowercase_keys(filename)
