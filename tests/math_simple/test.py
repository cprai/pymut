import mymath

def addsub(a, b):
    return mymath.subtract(mymath.add(a, b), b) == a

def comparepower(a, b):
    return mymath.power(a, b) == mymath.intPower(a, b)

def incdec(a):
    return increment(decrement(a)) == a

assert mymath.add(3, 4) == 7
assert mymath.subtract(3, 4) == -1
assert mymath.power(4, 3) == 48
assert mymath.intPower(5, 3) == 125
assert mymath.increment(5) == 6
assert mymath.decrement(5) == 4

assert addsub(4, 2) == True
assert addsub(4324, 25345) == True

assert comparePower(3, 2) == True
assert comparePower(123, 5) == True

assert incdec(4) == True
assert incdec(234) == True