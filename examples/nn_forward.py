# ── Neural Net Forward Pass ── Python ──
# Same network: 4 inputs → 3 hidden (ReLU) → 2 output (Softmax) → CE Loss
import numpy as np                      # <-- required
import math                             # <-- required for pure softmax

X  = np.array([0.5, 0.2, 0.8, 0.1])

W1 = np.array([[ 0.2, -0.4,  0.1,  0.3],
               [ 0.5,  0.1, -0.2,  0.4],
               [-0.1,  0.3,  0.6, -0.3]])
b1 = np.array([0.1, -0.1, 0.2])

W2 = np.array([[ 0.4, -0.3,  0.2],
               [-0.1,  0.5,  0.3]])
b2 = np.array([0.05, -0.05])

# Layer 1: linear + ReLU
z1 = W1 @ X + b1
a1 = np.maximum(0, z1)               # ReLU — no built-in, use np.maximum

# Layer 2: linear + Softmax
z2 = W2 @ a1 + b2
e     = np.exp(z2 - z2.max())        # Softmax — must implement manually
probs = e / e.sum()

# Cross-entropy loss — must implement manually
y_true = np.array([1.0, 0.0])
loss = -np.sum(y_true * np.log(np.clip(probs, 1e-12, 1.0)))

print("=== Python — Neural Net Forward Pass ===")
print("Input:        ", X.tolist())
print("Hidden (ReLU):", [round(v, 4) for v in a1.tolist()])
print("Output probs: ", [round(v, 4) for v in probs.tolist()])
print("Prediction:   class", int(np.argmax(probs)))
print("CE Loss:      ", round(float(loss), 6))
print("")
print("Import lines  : 2  (numpy, math)")
print("AI fn calls   : np.maximum, np.exp, np.sum, np.argmax, np.clip — all manual")
