def check(nums):
  safeCount = 0
  isIncreasing = False
  isDecreasing = False
  isSafe = True
  for i in range(len(nums) - 1):  
    num1 = nums[i] 
    num2 = nums[i + 1]  

    diff = num2 - num1

    if diff < -3 or diff > 3 or diff == 0:
      isSafe = False
      break

    if diff > 0:
      isIncreasing = True
    elif diff < 0:
      isDecreasing = True

    if isDecreasing and isIncreasing:
      isSafe = False
      break

    if isSafe:
      safeCount += 1 
    return safeCount
    

with open('sample.txt', 'r') as file:
  safeCount = 0
  for line in file:
    line = line.strip()  

    nums = [int(x) for x in line.split()]  

    result = check(nums)   
    safeCount += result 

print(f"Total safe reports: {safeCount}")

