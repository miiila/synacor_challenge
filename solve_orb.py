
maze = {
    (0,0): "O",
    (0,1): "-",
    (0,2): 9,
    (0,3): "*",
    (1,0): "+",
    (1,1): 4,
    (1,2): "-",
    (1,3): 18,
    (2,0): 4,
    (2,1): "*",
    (2,2): 11,
    (2,3): "*",
    (3,0): "*",
    (3,1): 8,
    (3,2): "-",
    (3,3): 1,
}


def move(curPos, curOp, curW, path):

    newCurOp = curOp
    newCurW = curW
    if maze[curPos] != "O":
        if maze[curPos] == "*":
            newCurOp = mult
        elif maze[curPos] == "+":
            newCurOp = add
        elif maze[curPos] == "-":
            newCurOp = sub
        else:
            newCurW = curOp(curW, maze[curPos])

    if curPos == (3,3) and newCurW:
        print(len(path), path, newCurW)

    r,c = curPos
    newPos = []
    if r-1 >= 0:
        newPos.append((r-1, c))
    if c-1 >= 0:
        newPos.append((r, c-1))
    if r+1 < 4:
        newPos.append((r+1, c))
    if c+1 < 4:
        newPos.append((r, c+1))

    for newP in newPos:
        if maze[newP] == "O" or len(path) > 12:
            continue
        if newP == (3,3) and curPos == (3,2) and curW != 31:
            continue
        if newP == (3,3) and curPos == (2,3) and curW != 30:
            continue
        newPath = path[:]
        newPath.append(newP)
        move(newP, newCurOp, newCurW, newPath)


def mult(a,b):
    return a*b

def add(a,b):
    return a+b

def sub(a,b):
    return a-b


move((0,0), None, 22, [])
