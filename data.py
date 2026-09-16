from datasets import load_dataset

ds = load_dataset("agi-noobs/chess-sft-20k")
ds = ds["train"].select_columns([
    "board_ascii",
    "legal_moves_uci",
    "side_to_move"
]).select(range(100))

def modify_row(row):
    lines = row["board_ascii"].strip().splitlines()

    # Remove top/bottom coordinate lines
    lines = lines[1:-1]

    # Remove row number and spaces from each board row
    lines = [
        line[1:].replace(" ", "").replace(".", "-")
        for line in lines
    ]

    row["board_ascii"] = "\n".join(lines)
    return row

ds = ds.map(modify_row)

ds.to_csv("chess_positions.csv")
