import time

iters = 1000000

start = time.time()
for _ in range(iters):
    pass
secs = time.time() - start
ops = iters / secs

print("Python VM's 'clock' speed is {:.3f} GHz".format(ops / 1000000000))
