from itertools import combinations
import re
import sys


if len(sys.argv) == 1:
    print("Please provide the amount to find")
    sys.exit(1)

try:
    target = float(sys.argv[1])
except ValueError:
    print(f"Failed to parse '{sys.argv[1]}' as a number")
    sys.exit(1)

lines: list[str] = []

print(
    "Please provide the list of expenses, one per line. Press Ctrl-D when you're done."
)
while True:
    try:
        line = input()
        lines.extend(re.sub(r"[\s]+", " ", line.strip()).splitlines())
    except EOFError:
        break
    if line.strip().lower() == "stop":
        break

amounts = []


for line in lines:
    line = re.sub(r"[\n\s]+", " ", line.strip())
    for term in line.split():
        if line.strip() == "":
            continue
        try:
            amount = float(term)
            if amount != 0.0:
                amounts.append(amount)
        except ValueError:
            continue

for amount in amounts:
    if amount < 0:
        if amount * -1 in amounts:
            # print(amount, amount * -1)
            amounts.remove(amount)
            amounts.remove(amount * -1)


if target in amounts:
    print(f"Found {target} in the list")
    sys.exit(0)


found = False
foundsets = []
for i in range(2, len(amounts)):
    for comb in combinations(amounts, i):
        if sorted(comb) in foundsets:
            continue
        if sum(comb) == target:
            print(
                f"Found the following combination that works: {','.join([str(a) for a in comb])}"
            )
            for item in comb:
                for line in lines:
                    if str(item) in line:
                        print(f"{line}")
            print("#" * 50)
            foundsets.append(sorted(comb))
            found = True
            # sys.exit(0)
if not found:
    print("Couldn't find a combination that works!")
    print("Had these amounts:")
    print("\n".join([str(a) for a in amounts]))
