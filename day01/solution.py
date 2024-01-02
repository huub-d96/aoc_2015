#Load the data
f = open('data.txt', 'r')
data = f.read()
f.close()

#data = '()())'

#Loop over inputs
loc = 0
basement = None
for i,c in enumerate(data.strip()):
    loc += 1 if c == '(' else -1

    if loc == -1 and not basement:
        basement = i+1

print('Part 1:', loc)
print('Part 2:', basement)
