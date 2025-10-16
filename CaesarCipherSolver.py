# Sub Programs
def caesarCipherSolver(ciphertext, shift):
    plaintext = ""
    for char in ciphertext:
        if char.isalpha():
            base = ord('A') if char.isupper() else ord('a')
            plaintext += chr((ord(char) - base - shift) % 26 + base)
        else:
            plaintext += char
    return plaintext
ciphertext = input("Enter the ciphertext: ")
# Main Program
for i in range (0,26):
    decoded_text = caesarCipherSolver(ciphertext, i)
    print("\nDecoded text:", decoded_text)
