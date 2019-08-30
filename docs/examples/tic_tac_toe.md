# Tic Tac Toe

```
import { AskInteger, Print } from "console"
import { RandomInt } from "math"

fn Main(): symbol {
    let mut board = [i32; 9] { :none }
    Loop(9) {
        | board -> PlayRound(it)
    }
    Print(| board -> Pretty)
    Print("Done.")
    return :ok
}

fn PlayRound(mut board: symbol[,], round: i32) {
    let who = if round & 1
        then { :ai }
        else { :player }
    let [x, y] = | board -> GetMove(who)
    board[x, y] = who
}

fn GetMove(board: symbol[,], who: symbol): i8[2] {
    eject GetMove(board, who)
    
    Print(| board -> Pretty)

    let [x, y]
        = switch who {
            :ai { GetMoveAI() }
            else { GetMovePlayer() }
        }

    if !board[x, y]? {
        return [x, y]
    } else {
        if who == :player {
            Print("Invalid move.")
        }
        fail
    }
}

fn GetMoveAI(): i8[2] {
    [RandomInt(0, 3), RandomInt(0, 3)]
}

fn GetMovePlayer(): i8[2] {
    [AskInteger("Move X"), AskInteger("Move Y")]
}

fn Pretty(board: symbol[,]): string {
    | board
        -> Map { it -> ToString }
        -> String::Join("\n", " ")
}

export Main
```