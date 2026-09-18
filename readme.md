# Simple chess lib

## Main idea: functional programming

This chess engine deals in game states where each game state represents a valid state of a game. 
When you make a move, you pass the game as an argument and the function spits out a new valid game state,
keeping the original one intact.

In viggoskj-chess-cli you can see examples of move making. The game state does nto contains if its check, 
checkmate, or stalemate, you have to use the get_check_state function for that.

Error handling is done by function returning Result<_, ChessError> where ChessError is a enum
describing what when wrong.

The main ways to move a piece in this engine would be to use the *legal_moves_bitboard* function to get a bitboard 
of valid moves for the piece on the square selected. You can then directly use the bitboard for rendering
or user the bitboard_iterator function to generate an iterator for that bitboard, if you only want the 
truthy squares you can filter by that with the iter.filter() function