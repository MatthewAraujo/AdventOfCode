list1 = []
list2 = []

with open('input.txt', 'r') as file:
  for line in file:
    line = line.strip()
    col1, col2 = line.split()
    list1.append(int(col1))
    list2.append(int(col2))

list1.sort()
list2.sort()

frequency = {}

for item in list2:
  frequency[item] = frequency.get(item, 0) + 1

similarity_sum = 0

for num in list1:
  similarity_sum += num * frequency.get(num, 0)

print("\nTotal sum:", similarity_sum)
