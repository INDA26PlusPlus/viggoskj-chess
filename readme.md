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

the bitboard reresentation is so most significant bit is a8 and when going from significant to insignificant bit it goes row by row ending with the least significant bit at h8.

the general flow of this lib is to create a new game with the *create_game()* function to get the start position of a game. Then use the *play_move()* function to play a valid move. There are basic and andvanced moves, for any move except promotions you can use the *BasicMove* type, for promotions you need to use the *AdvancedMove* type with the promotion variant. Then after playing a move you can use the *get_check_state()* function get if the game is in check, checkmate, stalemate, or neither.

If you have a gui and need to display possible moves you can use the *possible_legal_moves()* function to get a vector of all moves represented as BasicMoves (promotions are listed as basic moves so those cannot be passed to move but made into a promotion move). You can also use the *possible_legal_move_bitboards()* to instead get possible moves in a bitboard representation