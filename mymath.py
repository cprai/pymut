def add(a, b):
    return a + b

def subtract(a, b):
    return a - b

def power(a, b):
    return a ** b

def intPower(a, b):
    result = 1

    while b:
        result = result * a
        b = b - 1

    return result

def increment(a):
    return a + 1

def decrement(a):
    return a - 1