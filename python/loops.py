import timeit

# Compare two orders of nested loops with the same total number of
# iterations. Big-O is the same; only the constant factor differs.
# The difference is how many times the inner loop is started. Each start
# creates a range object and an iterator (a fixed cost).


def version_a(outer, inner):
    cnt = 0
    for i in range(outer):      # inner loop is started `outer` times
        for j in range(inner):
            cnt += 1
    return cnt


def version_b(outer, inner):
    cnt = 0
    for j in range(inner):      # inner loop is started `inner` times
        for i in range(outer):
            cnt += 1
    return cnt


def bench(f, outer, inner, number=10_000, repeat=5):
    # min of several repeats reduces noise from other processes
    return min(timeit.repeat(lambda: f(outer, inner), number=number, repeat=repeat))


# 100 x 10 = 1,000 additions
print("100 x 10:   A =", bench(version_a, 100, 10))
print("100 x 10:   B =", bench(version_b, 100, 10))

# 1000 x 10 = 10,000 additions (larger body work, gap should shrink)
print("1000 x 10:  A =", bench(version_a, 1000, 10))
print("1000 x 10:  B =", bench(version_b, 1000, 10))

# Results (CPython, my machine, best of 5, 10,000 calls each):
# 100 x 10 (1,000 additions):
#   A: 0.197 s, B: 0.139 s -> B about 30% faster
# 1000 x 10 (10,000 additions):
#   A: 2.030 s, B: 1.786 s -> B about 12% faster
#
# Same number of additions, but B starts the inner loop fewer times
# (10 vs 100, or 10 vs 1000), so it pays the fixed start-up cost less
# often. The relative gap shrinks as the body work grows.
# Big-O is unchanged: both do a*b iterations, O(a*b).
