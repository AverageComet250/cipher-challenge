from collections import Counter
import json
import enchant
import nltk


words3 = [word for word, freq in Counter(word.lower() for word in nltk.corpus.brown.words() if word.isalpha() and len(word) == 3).most_common(500)]


def printd(counter):
    for item in counter:
        print(f"\033[95m{item[0]}: \033[91m{item[1]}\033[00m", end=", ")
    print("\n")

def streplace(old, new, index):
    return f"{old[:index]}{new}{old[index+1:]}"


alphabet = "abcdefghijklmnopqrstuvwxyz"
key = "**************************"
new_key = "**************************"
print("ENTER CIPHER:")
cipher = input()
print()

freq_count = Counter(cipher).most_common()
letter_freq = []

for i, letter in enumerate(freq_count):
    if letter[0].isalpha():
        letter_freq.append(letter)

printd(letter_freq)

words = cipher.split()

threes = [word for word in words if len(word) == 3]
twos = [word for word in words if len(word) == 2]
ones = [word for word in words if len(word) == 1]

printd(Counter(threes).most_common())
printd(Counter(twos).most_common())
printd(Counter(ones).most_common())

conjunctions = []

for i, word in enumerate(words):
    if word.endswith(","):
        conjunctions.append(words[i+1])

printd(Counter(conjunctions).most_common())


# round based substitution

# start by guessing for the, then for because, then for i and a

print("Guessing for the")

for word, _ in Counter(threes).most_common():
    if word in conjunctions:
        continue

    print(word)
    key = streplace(key, word[0], 19)
    key = streplace(key, word[1], 8)
    key = streplace(key, word[2], 4)
    break


print("Now Guessing but")

for word, _ in Counter(conjunctions).most_common():
    if len(word) != 3 or word[2] != key[19]:
        continue
    print(word)
    key = streplace(key, word[0], 1)
    key = streplace(key, word[1], 20)
    # key = streplace(key, word[2], 19)


#    print("Now Guessing because")
#
#    for word, _ in Counter(conjunctions).most_common():
    #        if len(word) != 7 or word[1] != key[4]:
    #        continue
    #    print(word)
    #    key = streplace(key, word[0], 1)
    #    # key = streplace(key, word[1], 4)  # another e
    #    key = streplace(key, word[2], 3)
    #    key = streplace(key, word[3], 0)
    #    key = streplace(key, word[4], 21)
    #    key = streplace(key, word[5], 19)
    #    # key = streplace(key, word[6], 4)  # another e
    #
    #    print(key)
    #
    #print(key)


#cracked = cipher
    #for i, letter in enumerate(alphabet):
    #if letter == "*":
    #    continue
#cracked = cracked.lower().replace(letter, key[i])

# This code block was GPT generated


#result = ""
    #for ch in cipher:
    #if ch.isalpha():
    #    upper = ch.upper()
    #    rep = {alphabet[i]: key[i] for i in range(26)}.get(upper, "*")
    #    result += rep if ch.isupper() else rep.lower()
    #else:
#    result += ch

# Said Code block does not work lmao

result = ""
for letter in cipher:
    if letter.isalpha():
        try:
            letter = alphabet[key.index(letter)]
        except ValueError:
            letter = "*"
    result += letter


print(result)
print()
print(key)
