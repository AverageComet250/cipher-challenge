with open("./cipher-rs/data/words.txt", "r") as words:
    data = "".join([line.rstrip() for line in words]).rstrip()
    eval("words = {}".format(data))
    # eval("print('hellow')")
