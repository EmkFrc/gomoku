Two players take turns placing stones of their color on an intersection of the board,
and the game ends when one player manages to align five stones. Sometimes, only
an alignment of 5 can win, and sometimes 5 or more is okay. In the context of this
projet, we will consider 5 or more to be a win.
• There are different interpretations on what the board size for Gomoku should be.
In the context of this project, Gomoku will be played on a 19x19 Goban, without
limit to the number of stones.
• There are a great many additional rules to Gomoku (Google it!) aimed at making
the game more fair (regular Gomoku is proven to be unfair, a perfect first player
wins 100% of the time) and more interesting.
• Capture (As in the Ninuki-renju or Pente variants) : You can remove a pair of your
opponent’s stones from the board by flanking them with your own stones (See the
appendix). This rule adds a win condition : If you manage to capture ten of your
opponent’s stones, you win the game.
• Endgame Capture:
◦ A player who manages to line up five stones wins only if the opponent cannot
break this line by capturing a pair.
◦ If the player has already lost four pairs and the opponent can capture one
more, the opponent wins by capture.
◦ If there is no possibility of this happening, there is no need to continue the
game.
• No double-threes : It is forbidden to play a move that introduces two free-three
alignments, which would guarantee a win by alignment (See the appendix).

Appendix (summary of en.subject.pdf, chapter VI)
-------------------------------------------------

Captures
• A capture flanks a PAIR of opponent stones: X O O X. Playing the last X removes
  the two O. The freed intersections can be played again as if never occupied.
• Only pairs: never a single stone, never 3 stones or more in a row.
• One cannot move into a capture: placing your own stone between two enemy stones
  (O X _ O, then X plays in _) is safe.

Free-threes
• A free-three is an alignment of three stones that, if not immediately blocked,
  allows an indefensible alignment of four (four stones with both ends free).
• Examples given by the subject: . X X X .   and   . X . X X .
• A double-three is a move that introduces two free-threes at the same time.
  It is forbidden.
• If an opponent stone blocks one end of one of the threes, the move becomes legal.
• Exception: introducing a double-three by capturing a pair is NOT forbidden.

Bonus (only graded if the mandatory part is perfect)
• Choosing the rules at game start, e.g. openings Standard, Pro, Swap, Swap2.


Implementation choices (back/src/main.rs)
------------------------------------------

• Free-three, strict reading of the definition: three stones form a free-three only
  if adding ONE stone gives exactly four aligned with both ends empty.
  So  O . X X X . O  is NOT a free-three (every possible four has a blocked end),
  and a three against the board edge is not free either.
• Only the threes that go through the stone just played count ("introduces").
• Breaking a line: the opponent breaks a five only if, after his capture, no five
  remains. Capturing the end of a line of 6 leaves 5, so it breaks nothing.
• If the opponent does not break a breakable five and lines up five himself instead,
  the first five wins (it was there first and was not broken).
• Draw: board full with no winner.
