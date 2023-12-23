# Pythonで素数を100個生成

# 素数判定を行う関数
def is_prime(n):
    for i in range(2, n):
        if n % i == 0:
            return False
    return True

# count個の素数を生成する関数
def get_primes(count):
    primes = []
    n = 2
    while len(primes) < count:
        if is_prime(n):
            primes.append(n)
        n += 1
    return primes

# 100個の素数を生成
primes = get_primes(100)
print(primes)
