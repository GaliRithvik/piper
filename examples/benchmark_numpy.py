# Python + NumPy
import numpy as np

# Benchmark 1: Loop (vectorized)
total = int(np.sum(np.arange(1, 100001, dtype=np.int64)**2))
print("Sum of squares:", total)

# Benchmark 2: Dot product, 200 times
a = np.arange(1, 501, dtype=np.float64)
b = np.arange(501, 1001, dtype=np.float64)
result = 0
for _ in range(200):
    result = np.dot(a, b)
print("Dot product:", int(result))

# Benchmark 3: 20x20 matmul, 30 times
row = np.arange(1, 21, dtype=np.float64)
M = np.tile(row, (20, 1))
mm_result = M
for _ in range(30):
    mm_result = np.matmul(M, M)
print("Matmul done, top-left:", int(mm_result[0][0]))
