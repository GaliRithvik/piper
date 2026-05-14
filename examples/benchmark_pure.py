# Pure Python — no libraries
# Benchmark 1: Loop
total = 0
for i in range(1, 100001):
    total += i * i
print("Sum of squares:", total)

# Benchmark 2: Dot product, 200 times
a = list(range(1, 501))
b = list(range(501, 1001))
result = 0
for _ in range(200):
    result = sum(x * y for x, y in zip(a, b))
print("Dot product:", result)

# Benchmark 3: 20x20 matmul, 30 times
row = list(range(1, 21))
M = [row[:] for _ in range(20)]
def matmul(A, B):
    n = len(A)
    return [[sum(A[i][k]*B[k][j] for k in range(n)) for j in range(n)] for i in range(n)]
mm_result = M
for _ in range(30):
    mm_result = matmul(M, M)
print("Matmul done, top-left:", mm_result[0][0])
