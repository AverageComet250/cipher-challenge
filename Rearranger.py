# Subprograms
def rearrangeChunks(cipherText, chunkLength, newOrder):
    # Checks for the format for new order
    order = [int(x) for x in str(newOrder)]
    if sorted(order) != list(range(1, chunkLength + 1)):
        raise ValueError("New order must contain each number from 1 to chunk length exactly once.")
    plainText = ""
    # Rearranges    
    for i in range(0, len(cipherText), chunkLength):
        chunk = cipherText[i:i + chunkLength]
        if len(chunk) < chunkLength:
            chunk = chunk.ljust(chunkLength)
        rearranged = ''.join(chunk[o - 1] for o in order)
        plainText += rearranged
    return plainText.strip()

# Main Program
cipherText = input("Enter your text to be rearranged: ")
chunkLength = int(input("Enter the length of each chunk which is to be rearranged: "))
newOrder = input("Enter the rearranged order by index starting 1: ")
plainText = rearrangeChunks(cipherText, chunkLength, newOrder)
print("The plaintext is:" )
print(plainText)
