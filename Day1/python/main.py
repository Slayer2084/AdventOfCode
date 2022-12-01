import numpy as np

def main():
    with open("C:/Users/Paul/PycharmProjects/AdventOfCode/Day1/data.txt") as f:
        lines = [line.rstrip('\n') for line in f]
    result = []
    current = 0
    for line in lines:
        if line != "":
            current += int(line)
        else:
            result.append(current)
            current = 0
    print("Solution of the first part is: " + str(np.max(result)))
    
    ind = np.argpartition(result, -3)[-3:]
    sum = 0
    for index in ind:
        sum += result[index]
    print("Solution of the second part is: " + str(sum))
        

if __name__ == "__main__":
    main()