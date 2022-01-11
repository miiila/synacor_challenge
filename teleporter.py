import sys
sys.setrecursionlimit(2500)

stack = []

counter = 0
def f6027(reg):
    if reg[0] > 0:
        if reg[1] > 0:
            stack.append(reg[0])
            reg[1] = reg[1] - 1 
            f6027(reg)

            reg[1] = reg[0]
            reg[0] = stack.pop()
            reg[0] = reg[0] - 1
            return f6027(reg)
        else:
            reg[0] = reg[0] - 1
            reg[1] = reg[7]
            f6027(reg)
    else:
        reg[0] = reg[1] + 1
        # print("UNWIND", stack, reg)
        return


regs = {0: 3, 1: 1, 7:14}
f6027(regs)
# r = f6027_iter(regs)
print(regs)
