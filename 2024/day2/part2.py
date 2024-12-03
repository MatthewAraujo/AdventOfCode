def check(nums):
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

    return isSafe


def canBeMadeSafe(nums):
    if check(nums):
      return True

    for i in range(len(nums)):
      newNums = nums[:i] + nums[i+1:]  
      if check(newNums):  
        return True

    return False


with open('input.txt', 'r') as file:
  safeCount = 0
  for line in file:
    line = line.strip()

    nums = [int(x) for x in line.split()]

    if check(nums) or canBeMadeSafe(nums): 
      safeCount += 1

print(f"Total safe reports: {safeCount}")
