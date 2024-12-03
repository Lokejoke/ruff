a[1:3] = [1, 2]
a[slice(1)] = [1, 3]

a[:0], a[-1:] = prefix, suffix

a[:] = complete_replacement

a[0:0] = prepend

a[0] = 1


size = 3
board = [["_"] * size for _ in range(size)]

board[0:1][1:2] = "X"

board[0] = board[0:1] = board[1:2] = [board[0][:], board[1][:], board[2][:]] = board