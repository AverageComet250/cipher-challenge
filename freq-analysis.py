from collections import Counter

def printd(counter):
    for item in counter:
        print(f"\033[95m{item[0]}: \033[91m{item[1]}\033[00m", end=", ")
    print()
    print()


alphabet = "abcdefghijklmnopqrstuvwxyz"
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

for word in words:
    if len(word) == 3:
        threes.append(word)

printd(Counter(threes).most_common())

conjunctions = []

for i, word in enumerate(words):
    if word.endswith(","):
        conjunctions.append(words[i+1])

printd(Counter(conjunctions).most_common())
