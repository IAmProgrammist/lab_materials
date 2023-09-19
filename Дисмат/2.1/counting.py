y = [0, 0, 0, 0, 0, 0, 0.003, 0.016, 0.111, 1.043, 11.39, 137.71]
x= [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
sumxiyi = 0
sumyi = 0
sumxi = 0
sumxisquare = 0
def fac(n):
    if n <= 1:
        return 1
    return fac(n - 1) * n

for i in range(0, len(x)):
    sumxiyi += fac(x[i]) * y[i]

for i in range(0, len(x)):
    sumyi += y[i]

for i in range(0, len(x)):
    sumxi += fac(x[i])

for i in range(0, len(x)):
    sumxisquare += fac(x[i]) ** 2

print(sumxiyi, sumyi, sumxi, sumxisquare)

a = (sumxiyi - sumyi * sumxi) / (sumxisquare - sumxi * sumxi)

print(a)

b = sumyi - a * sumxi

print(b)

print(sumxiyi / sumxisquare)
