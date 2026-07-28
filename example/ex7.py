import torch

m: int
k: int
p: int
n: int

A: torch.Tensor[m, k] = torch.tensor([[2, 3, 4]], dtype="f16")
B: torch.Tensor[p, n]

C = torch.matmul(A, B)

D = A + B
