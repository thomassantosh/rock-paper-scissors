import random
from collections import Counter

total_items = ['rock', 'paper', 'scissors']


def outcome(a, b):
    if a == b:
        resultSet = "Same value. No winner."
    elif a == 'rock' and b == 'paper':
        resultSet = "Paper wins. Rock loses."
    elif a == 'rock' and b == 'scissors':
        resultSet = "Rock wins. Scissors loses."
    elif a == 'paper' and b == 'rock':
        resultSet = "Paper wins. Rock loses."
    elif a == 'scissors' and b == 'rock':
        resultSet = "Rock wins. Scissors loses."
    elif a == 'paper' and b == 'scissors':
        resultSet = "Scissors wins. Paper loses."
    elif a == 'scissors' and b == 'paper':
        resultSet = "Scissors wins. Paper loses."
    else:
        resultSet = 'No determination'
    return resultSet


# Values after 'x' iterations
firstValue = []
secondValue = []
finalOutcome = []

x = 1000000

for i in range(0, x):
    a = random.choice(total_items)
    b = random.choice(total_items)
    c = outcome(a, b)
    firstValue.append(a)
    secondValue.append(b)
    finalOutcome.append(c)

finalSet_keys = Counter(finalOutcome).keys()
finalSet_values = Counter(finalOutcome).values()

ratios = [number / x for number in finalSet_values]
print(finalSet_keys)
print(ratios)
