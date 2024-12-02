list1 = []
list2 = []

with open('input.txt', 'r') as file:
  for line in file:
    line = line.strip()
    col1, col2 = line.split()
    list1.append(int(col1))
    list2.append(int(col2))

print("Lista 1:", list1)
print("Lista 2:", list2)


list1.sort()
list2.sort()

minior = []
total = 0
for i in range(len(list1)):
  diff = abs((list1[i]) - (list2[i]))
  total += diff
  minior.append(diff)

print(sum(minior))
print(total)