Tic tac toe written in Rust.
This is an alternative version of the game Tic Tac Toe, where the board consists of 25 cells (5x5) and in order to win, you need to have four consecutive cells with your mark.
The user can choose who plays first - them or the computer.

The computer uses two strategies, depending on where the user plays their first move. If the computer is first, the first strategy is always used.
The two strategies consist of the following moves, in prioritized order:
- Winning move - if there are three consecutive cells with the computer's mark, and next to them there is a free cell, the computer plays the next move there and wins;
- Trap move - trap move is played when the two cells adjacent to the center column have the computer's mark, the center cell in that row is empty, and the two edge cells on the two sides is empty. The computer plays the next move and places its mark in the center cell. That way, regardless of which edge cell the human chooses to place their mark on, the computer will place its mark on the other one, and will win;
- Double sided trap move - similar to the Trap move, but the center cell and the two adjacent cells form a corner;
- Risk move - if none of the previous moves can be used, the risk for each cell is calculated - that consists of calculating the amount of cells with the opponent's mark in the current row, column and two diagonals. The cell with the highest risk is chosen and the computer places its mark there;
- Attack move - this is the final move, which is played when the highest risk is lower than or equal to one. That way the computer isn't playing defence when there is no need for it. The attack move discovers the best fit cell to place the computer's mark which will bring the computer one step closer to winning.