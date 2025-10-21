from collections import Counter
import json
import enchant
import nltk



with open("./cipher-rs/data/words.txt", "r") as words:
    data = "".join([line.rstrip() for line in words]).rstrip()
    eval(f"data = {data}")
    print(data)
    words3 = []
    for word in data: # replace with generator
        print(word)
        if len(word) == 3:
            words3.append(word)

# print(data)
print(words3)

def printd(counter):
    for item in counter:
        print(f"\033[95m{item[0]}: \033[91m{item[1]}\033[00m", end=", ")
    print()
    print()


alphabet = "abcdefghijklmnopqrstuvwxyz"
key = "abcdefghijklmnopqrstuvwxyz"
cipher = input()
print()

freq_count = Counter(cipher).most_common()
letter_freq = []

for i, letter in enumerate(freq_count):
    if letter[0].isalpha():
        letter_freq.append(letter)

printd(letter_freq)

words = cipher.split()

# rewrite with generator

threes = []
twos =[]
ones = []

for word in words:
    if len(word) == 3:
        threes.append(word)
    elif len(word) == 2:
        twos.append(word)
    elif len(word) == 1:
        ones.append(word)


printd(Counter(threes).most_common())
printd(Counter(twos).most_common())
printd(Counter(ones).most_common())

conjunctions = []

for i, word in enumerate(words):
    if word.endswith(","):
        conjunctions.append(words[i+1])

printd(Counter(conjunctions).most_common())

# round based substitution

if _has_enchant == False:
    exit()

for word in threes:
    if word in conjunctions:
        pass

    print(word)

    break


print(key)
