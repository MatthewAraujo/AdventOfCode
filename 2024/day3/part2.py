import re

with open('sample.txt', 'r') as file:
  content = file.read()


nums = re.findall("mul\(\d+,\d+\)|do\(\)|don't\(\)", content) 


do = True
result = 0
for mul in nums:
  if mul == "do()":
    print("do?")
    do = True
  elif mul == "dont'()":
    do = False
  else:
    if do:
      value = mul.split("(")[1].strip(")")  
      a, b = value.split(",")
      result += int(a) * int(b)


print("Resultado final:", result)