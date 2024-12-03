import re

with open('input.txt', 'r') as file:
  content = file.read()


nums = re.findall("mul\(\d+,\d+\)", content) 


result = 0
for mul in nums:
    value = mul.split("(")[1].strip(")")  
    a, b = value.split(",")
    result += int(a) * int(b)


print("Resultado final:", result)