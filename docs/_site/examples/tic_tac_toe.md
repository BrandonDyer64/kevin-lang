# Tic Tac Toe

```
import Console from "console"
import { RandomInt } from "math"

fn symbol Main([string] args) {
    let mut board = [symbol; 3, 3] => :none;
    [1...9] loop i => board->PlayRound(i);
    board->Print();
    Console::Print("Done.");
    return :ok;
}

fn PlayRound(mut [symbol,] board, i32 round) =>
    <- board->Move()
    <- if round & 1
        then => :ai
        else => :player;

fn Move(mut [symbol,] board, symbol who) {
    let i8 [x, y] = switch who {
        :ai => board->MoveAI();
        default => board->MovePlayer();
    };
    if board[x, y] != :none return Move(board, who);
    board[x, y] = who;
}

fn [i8 2] MoveAI([symbol,] board) => [
    , RandomInt(0, 3)
    , RandomInt(0, 3)
];

fn [i8 2] MovePlayer([symbol,] board) {
    board->Print();
    return [
        , Console::AskInteger("Move X")
        , Console::AskInteger("Move Y");
    ];
}

fn Print([symbol,] board) =>
    -> board
    -> String::Join("\n", " ")
    -> Console::Print();

export Main;
```