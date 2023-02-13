TEST = "test"
INPUT = "input"

file = open(TEST, "r")
lines = file.readlines()

def pprint_bingo(bingos):
    for bingo in bingos:
        for row in bingo:
            print(row)
        print("")

def line_as_list(lines):
    ret = []
    value = ""

    for line in lines:
        for (i, char) in enumerate(line):

            if char == " " and value != "" or char == "\n" and value != "":
                ret.append(value)
                value = ""

            elif char in [str(i) for i in range(0, 10)]:
                value += char

            elif i == len(line):
                ret.append(value)

    return ret


def insert_bingo(line, bingos):
    bingo = [
            [],
            [],
            [],
            [],
            [],
            ]
    values = line_as_list(line)
    count = 0

    for array in bingo:
        for i in range(5):
            array.append((values[count], False))
            count += 1
    bingos.append(bingo)

def parse_numbers(line) -> list[str]:
    """ This is a comment """
    numbers: list[str] = []
    value = ""

    for char in line:

        if char in [str(i) for i in range(0, 10)]:
            value += char

        elif char == ',':
            numbers.append(value)
            value = ""

        elif char == '\n':
            numbers.append(value)
            
        else:
            print("should not happend")
    return numbers


count = 0
numbers = []
bingo_trays = []
one_bingo = []

for line in lines:
    if count == 0:
        numbers = parse_numbers(line)
    else:
        if line != "\n":
            one_bingo.append(line)
        if len(one_bingo) == 5:
            insert_bingo(one_bingo, bingo_trays)
            one_bingo = []
    count += 1


pprint_bingo(bingo_trays)
