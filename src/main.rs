use std::io;


#[derive(PartialEq)]
enum PlayerType {
    Player,
    Computer
}

#[derive(Clone, PartialEq, Debug)]
enum PlayerPiece {
    X,
    O,
}

#[derive(PartialEq, Debug)]
enum GameState {
    Playing,
    Ended
}


impl Into<char> for PlayerPiece {
    fn into(self) -> char {
        if self == PlayerPiece::X  {
            'X'
        } else {
            'O'
        }
    }
}

impl From<char> for PlayerPiece {
    fn from(ch: char) -> Self {
        if ch == 'X' {
            PlayerPiece::X
        } else {
            PlayerPiece::O
        }
    }
}



fn draw_board(state: [char; 9]) {
    println!("");
    println!("-------------");
    println!("| {} | {} | {} |", state[0], state[1], state[2]);
    println!("-------------");
    println!("| {} | {} | {} |", state[3], state[4], state[5]);
    println!("-------------");
    println!("| {} | {} | {} |", state[6], state[7], state[8]);
    println!("-------------");
    println!("");
}


fn ask_player_piece() -> PlayerPiece {
    println!("Choose piece?");
    println!("1.) X 2.) O");
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
       Ok(_) if buffer.trim().to_string() == '1'.to_string() => {
           PlayerPiece::X
       },
       Ok(_) if buffer.trim().to_string() == '2'.to_string() => {
           PlayerPiece::O
       },
       _ => {
           ask_player_piece()
       }
    }
    
}


fn ask_whos_first() -> PlayerType {
    println!("Welcome to Tic Tac Toe!");
    println!("Who goes first?");
    println!("1.) You 2.) Computer");
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) if buffer.trim().to_string() == '1'.to_string() => {
            PlayerType::Player
        },
        Ok(_) if buffer.trim().to_string() == '2'.to_string() => {
            PlayerType::Computer
        },
        _ => ask_whos_first()

    }
}


fn is_open(board: [char; 9], square: usize) -> bool {
    // Expects index in array positions
    if board[square] != PlayerPiece::O.into() && board[square] != PlayerPiece::X.into() {
        true
    } else {
        false
    }
}


fn computer_move(board: [char; 9], my_piece: PlayerPiece, their_piece: PlayerPiece) -> usize {
    // Todo minimax
    1
}


fn ask_move(board: [char; 9]) -> usize {
    println!("Where to place? (1-9)");
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
       Ok(_) => {
           match buffer.trim().to_string().parse::<usize>() {
               Ok(s) if s >= 1 && s <= 9 && is_open(board, s - 1)  => {
                   s
               },
               _ => {
                   draw_board(board);
                   ask_move(board)
               }
           }
       },
       Err(_) => {
           draw_board(board);
           ask_move(board)
       }
    }

}


fn fill_square(board: &mut [char; 9], player_move: usize, player_piece: PlayerPiece) {
    board[player_move - 1] = player_piece.into();
}

fn check_if_gameover(board: [char; 9], moves: usize) ->  (GameState, Option<PlayerPiece>) {
    if board[0] == board[1] && board[1] == board[2] {
        (GameState::Ended, Some(board[0].into()))
    } else if board[3] == board[4] && board[4] == board[5] {
        (GameState::Ended, Some(board[3].into()))
    } else if board[6] == board[7] && board[7] == board[8] {
        (GameState::Ended, Some(board[6].into()))
    } else if board[0] == board[4] && board[4] == board[8] {
        (GameState::Ended, Some(board[0].into()))
    } else if board[2] == board[4] && board[4] == board[6] {
        (GameState::Ended, Some(board[2].into()))
    } else if board[0] == board[3] && board[3] == board[6] {
        (GameState::Ended, Some(board[0].into()))
    } else if board[1] == board[4] && board[4] == board[7] {
        (GameState::Ended, Some(board[1].into()))
    } else if board[2] == board[5] && board[5] == board[8] {
        (GameState::Ended, Some(board[2].into()))
    } else if moves > 9 {
        (GameState::Ended, None)
    } else {
        (GameState::Playing, None)
    }
}


fn ask_if_play_again() -> bool {
    println!("Play again? 1.) Yes 2.) No");
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) if buffer.trim().to_string() == '1'.to_string() => {
            true
        },
        Ok(_) if buffer.trim().to_string() == '2'.to_string() => {
            false
        },
        _ => ask_if_play_again()
    }
}


fn play_game() {
    let mut turn = ask_whos_first();
    let mut winner: Option<PlayerPiece> = None;
    let player_piece = ask_player_piece();
    let computer_piece = if player_piece == PlayerPiece::X {
        PlayerPiece::O
    } else {
        PlayerPiece::X
    };
    let mut board = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut game_state = GameState::Playing;
    let mut moves = 0;

    draw_board(board);

    while game_state == GameState::Playing {
        moves += 1;
        if turn == PlayerType::Player {
            let player_move = ask_move(board);
            fill_square(&mut board, player_move, player_piece.clone());
            turn = PlayerType::Computer;
        } else {
            let computer_move = computer_move(board, computer_piece.clone(), player_piece.clone());
            fill_square(&mut board, computer_move, computer_piece.clone());
            turn = PlayerType::Player;
        }
        draw_board(board);
        let result = check_if_gameover(board, moves);
        game_state = result.0;
        winner = result.1;
        println!("{:?}", game_state);
    }
    match winner {
        Some(w) => {
            println!("{:?} wins!", w);
        },
        None => {
            println!("Tie!");
        }
    }
    if ask_if_play_again() {
        play_game()
    }
}

fn main() {
    play_game();
}
