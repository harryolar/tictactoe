use rand::Rng;
use std::io;
use std::process;

const EMPTY: usize = 0;
const HUMAN: usize = 1;
const AI: usize = 2;

enum GameStateMachine {
    Starting,
    HumanPlaying,
    AIPlaying,
    GameEnded,
    ExitGame,
}

fn get_user_move(max_move: usize) -> usize {
    let mut guess = String::new();
    
    loop {
        // clean string before use
        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Trim white space from user input
        let user_input = guess.trim();

        if let Ok(x) = user_input.parse::<usize>() {
            if x < max_move {
                return x;
            }
        } else if user_input == "Q" || user_input == "q" {
            println!("Quiting");
            process::exit(0x00);
        } else {
            println!("Could not parse input `{}`", user_input);
        }

        println!("Expected a number between 0 and {}.", max_move.saturating_sub(1));
        println!("Please try again!, or enter `q` to quit");
    }

    unreachable!()
}

fn get_human_move(m: &mut [usize], move_count: usize) {
    let number = get_user_move(m.len()); 
    
    if m[number] == EMPTY {
        m[number] = HUMAN;

        let human_moves: usize = check_player(m, HUMAN);
        if human_moves == 3 {
            print_board(m);
            println!("You won! Congrats ! Quiting the game ");
            process::exit(0x00);
        }
    } else {
        println!("The place is occupied")
    }
}

fn get_rand_not_occupied_move(board: &mut [usize]) -> Option<usize> {
    let mut move_count: usize = board.len();

    if move_count == 0 {
        return None; //board full
    }

    for i in board.iter_mut() {
        if *i != EMPTY {
            move_count = move_count - 1;
        }
    }

    if move_count == 0 {
        return None; //board full
    }

    let mut num: usize = 1;

    if move_count > 1 {
        num = rand::thread_rng().gen_range(1..move_count);
    }

    for (pos, i) in board.iter_mut().enumerate() {
        if *i == EMPTY {
            if num == 1 {
                return Some(pos);
            }
            num = num - 1;
        }
    }

    return None; //bad
}

fn get_ai_move(board: &mut [usize], move_count: usize) {
    if move_count == 0 {
        println!("My first shiny a$$ move:");
        // Generate random number in the range [0, 8]
        let num: usize = rand::thread_rng().gen_range(0..8);
        println!("{}", num);
        assert_eq!(board[num], EMPTY);
        board[num] = AI;
    } else {
        let human_moves: usize = check_player(board, HUMAN);
        if human_moves == 3 {
            print_board( board);
            println!("You won ! Quiting the game");
            process::exit(0x00);
        } else if human_moves == 2 {
            set_ai_next_move_when_2_occupied(board, HUMAN);
            let ai_local_moves: usize = check_player(board, AI);
             if ai_local_moves == 3 {
                print_board( board);
                println!("You lost. Exiting the game ");
                process::exit(0x00);
            }
        } else {
            let ai_moves: usize = check_player(board, AI);
            if ai_moves == 3 {
                println!("I won ! Quiting the game");
                process::exit(0x00);
            } else if ai_moves == 2 {
                set_ai_next_move_when_2_occupied(board, AI);
                let ai_local_moves: usize = check_player(board, AI);
                if ai_local_moves == 3 {
                    print_board(board);
                    println!("You lost. Exiting the game ");
                    process::exit(0x00);
                }
            } else {
                let m1 = get_rand_not_occupied_move(board);
                match m1 {
                    Some(x) => {
                        assert_eq!(board[x], EMPTY);
                        board[x] = AI;
                        let ai_local_moves: usize = check_player(board, AI);
                        if ai_local_moves == 3 {
                            print_board(board);
                            println!("You lost. Exiting the game ");
                            process::exit(0x00);
                        }
                    }
                    None => {
                        println!("Error: There is no move");
                        process::exit(0x00);
                    }
                }
            }
        }
    }
}

fn print_board(board: &[usize]) {
    println!("{} {} {}", board[0], board[1], board[2]);
    println!("{} {} {}", board[3], board[4], board[5]);
    println!("{} {} {}", board[6], board[7], board[8]);
}

fn set_ai_next_move_when_2_occupied(board: &mut [usize], player: usize) {
    if board[0] == player && board[1] == player && board[2] == EMPTY {
        assert_eq!(board[2], EMPTY);
        board[2] = AI;
    } else if board[0] == EMPTY && board[1] == player && board[2] == player {
        assert_eq!(board[0], EMPTY);
        board[0] = AI;
    } else if board[0] == player && board[1] == EMPTY && board[2] == player {
        assert_eq!(board[1], EMPTY);
        board[1] = AI;
    } else if board[3] == player && board[4] == player && board[5] == EMPTY {
        assert_eq!(board[5], EMPTY);
        board[5] = AI;
    } else if board[3] == EMPTY && board[4] == player && board[5] == player {
        assert_eq!(board[3], EMPTY);
        board[3] = AI;
    } else if board[3] == player && board[4] == EMPTY && board[5] == player {
        assert_eq!(board[4], EMPTY);
        board[4] = AI;
    } else if board[6] == player && board[7] == player && board[8] == EMPTY {
        assert_eq!(board[8], EMPTY);
        board[8] = AI;
    } else if board[6] == EMPTY && board[7] == player && board[8] == player {
        assert_eq!(board[6], EMPTY);
        board[6] = AI;
    } else if board[6] == player && board[7] == EMPTY && board[8] == player {
        assert_eq!(board[7], EMPTY);
        board[7] = AI;
    } else if board[0] == player && board[3] == player && board[6] == EMPTY {
        assert_eq!(board[6], EMPTY);
        board[6] = AI;
    } else if board[0] == EMPTY && board[3] == player && board[6] == player {
        assert_eq!(board[0], EMPTY);
        board[0] = AI;
    } else if board[0] == player && board[3] == EMPTY && board[6] == player {
        assert_eq!(board[3], EMPTY);
        board[3] = AI;
    } else if board[1] == player && board[4] == player && board[7] == EMPTY {
        assert_eq!(board[7], EMPTY);
        board[7] = AI;
    } else if board[1] == EMPTY && board[4] == player && board[7] == player {
        assert_eq!(board[1], EMPTY);
        board[1] = AI;
    } else if board[1] == player && board[4] == EMPTY && board[7] == player {
        assert_eq!(board[4], EMPTY);
        board[4] = AI;
    } else if board[2] == player && board[5] == player && board[8] == EMPTY {
        assert_eq!(board[8], EMPTY);
        board[8] = AI;
    } else if board[2] == EMPTY && board[5] == player && board[8] == player {
        assert_eq!(board[2], EMPTY);
        board[2] = AI;
    } else if board[2] == player && board[5] == EMPTY && board[8] == player {
        assert_eq!(board[5], EMPTY);
        board[5] = AI;
    } else if board[0] == player && board[4] == player && board[8] == EMPTY {
        assert_eq!(board[8], EMPTY);
        board[8] = AI;
    } else if board[0] == player && board[4] == EMPTY && board[8] == player {
        assert_eq!(board[4], EMPTY);
        board[4] = AI;
    } else if board[0] == EMPTY && board[4] == player && board[8] == player {
        assert_eq!(board[0], EMPTY);
        board[0] = AI;
    } else if board[2] == player && board[4] == player && board[6] == EMPTY {
        assert_eq!(board[6], EMPTY);
        board[6] = AI;
    } else if board[2] == player && board[4] == EMPTY && board[6] == player {
        assert_eq!(board[4], EMPTY);
        board[4] = AI;
    } else if board[2] == EMPTY && board[4] == player && board[6] == player {
        assert_eq!(board[2], EMPTY);
        board[2] = AI;
    } else {
        panic!("Couldn't find any moves");
    }
}

fn check_player(board: &[usize], player: usize) -> usize {
    if board[0] == player && board[1] == player && board[2] == player
        || board[3] == player && board[4] == player && board[5] == player
        || board[6] == player && board[7] == player && board[8] == player
        || board[0] == player && board[3] == player && board[6] == player
        || board[1] == player && board[4] == player && board[7] == player
        || board[2] == player && board[5] == player && board[8] == player
        || board[0] == player && board[4] == player && board[8] == player
        || board[2] == player && board[4] == player && board[6] == player
    {
        return 3;
    } else if board[0] == player && board[1] == player && board[2] == EMPTY
        || board[0] == EMPTY  && board[1] == player && board[2] == player
        || board[0] == player && board[1] == EMPTY  && board[2] == player

        || board[3] == player && board[4] == player && board[5] == EMPTY
        || board[3] == EMPTY  && board[4] == player && board[5] == player
        || board[3] == player && board[4] == EMPTY  && board[5] == player

        || board[6] == player && board[7] == player && board[8] == EMPTY
        || board[6] == EMPTY  && board[7] == player && board[8] == player
        || board[6] == player && board[7] == EMPTY  && board[8] == player

        || board[0] == player && board[3] == player && board[6] == EMPTY
        || board[0] == EMPTY  && board[3] == player && board[6] == player
        || board[0] == player && board[3] == EMPTY  && board[6] == player

        || board[1] == player && board[4] == player && board[7] == EMPTY
        || board[1] == EMPTY  && board[4] == player && board[7] == player
        || board[1] == player && board[4] == EMPTY  && board[7] == player

        || board[2] == player && board[5] == player && board[8] == EMPTY
        || board[2] == EMPTY  && board[5] == player && board[8] == player
        || board[2] == player && board[5] == EMPTY  && board[8] == player

        || board[0] == player && board[4] == player && board[8] == EMPTY
        || board[0] == EMPTY  && board[4] == player && board[8] == player
        || board[0] == player && board[4] == EMPTY  && board[8] == player

        || board[2] == player && board[4] == player && board[6] == EMPTY
        || board[2] == EMPTY  && board[4] == player && board[6] == player
        || board[2] == player && board[4] == EMPTY  && board[6] == player
    {
        return 2;
    } else if board[0] == player
        || board[1] == player
        || board[2] == player
        || board[3] == player
        || board[4] == player
        || board[5] == player
        || board[6] == player
        || board[7] == player
        || board[8] == player
    {
        return 1;
    }

    return 0;
}

fn main() {
    let mut guess = String::new();
    let mut m = [0; 9]; // tic tac toe board
    let mut count: usize = 0;
    let mut is_still_playing: bool = true;

    let mut game_state_machine: GameStateMachine = GameStateMachine::Starting;

    while is_still_playing {
        match game_state_machine {
            GameStateMachine::Starting => {
                guess.clear();
                println!("Hello, to tic tac toe . Do you want to start? Y/N or Q for quit");
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");

                if guess.trim() == "Q" || guess.trim() == "q" {
                    game_state_machine = GameStateMachine::ExitGame;
                } else if guess.trim() == "N" || guess.trim() == "n" {
                    game_state_machine = GameStateMachine::AIPlaying;
                } else {
                    game_state_machine = GameStateMachine::HumanPlaying;
                    println!("Please input your first move between 0 and 8 :");
                }
            }

            GameStateMachine::HumanPlaying => {
                get_human_move(&mut m, count);
                print_board(&mut m);
                count = count + 1;
                if count >= 9 {
                    game_state_machine = GameStateMachine::GameEnded;
                }
                game_state_machine = GameStateMachine::AIPlaying;
            }

            GameStateMachine::AIPlaying => {
                get_ai_move(&mut m, count);
                print_board(&mut m);
                count = count + 1;
                if count >= 9 {
                    game_state_machine = GameStateMachine::GameEnded;
                }
                game_state_machine = GameStateMachine::HumanPlaying;
            }

            GameStateMachine::GameEnded => {
                println!("Game over!");
                game_state_machine = GameStateMachine::Starting;
            }

            GameStateMachine::ExitGame => {
                is_still_playing = false;
                println!("Quiting");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(1, 1);

        // panic!("Error the output is greater than 8");
    }

    #[test]
    fn get_rand_not_occupied_move_test() {
        let mut m = [0; 9]; // tic tac toe board
        let mut count: u32 = 0;

        for n in 1..100 {
            let m1 = get_rand_not_occupied_move(&mut m);
            match m1 {
                Some(x) => {
                    if x > 8 {
                        panic!("Error the output is greater than 8");
                    };
                }
                None => {
                    panic!("Error the output is empty");
                }
            }
        }
    }

    #[test]
    fn get_rand_not_occupied_move_test2() {
        let mut m = [0; 9]; // tic tac toe board

        m[0] = 1;
        m[1] = 1;
        m[2] = 1;
        m[3] = 1;
        m[4] = 1;
        m[5] = 1;
        m[6] = 1;
        m[7] = 1;
        m[8] = 0;

        let mut count: u32 = 0;
        let m1 = get_rand_not_occupied_move(&mut m);
        match m1 {
            Some(x) => {
                assert_eq!(
                    x, 8,
                    "we are testing filling the 8th position when empty {} and {}",
                    x, 8
                );
            }
            None => {
                panic!("Error the output is empty");
            }
        }
    }

    #[test]
    fn get_rand_not_occupied_move_test3() {
        let mut m = [0; 9]; // tic tac toe board

        m[0] = 0;
        m[1] = 1;
        m[2] = 1;
        m[3] = 1;
        m[4] = 1;
        m[5] = 1;
        m[6] = 1;
        m[7] = 1;
        m[8] = 1;

        let mut count: u32 = 0;
        let m1 = get_rand_not_occupied_move(&mut m);
        match m1 {
            Some(x) => {
                assert_eq!(
                    x, 0,
                    "we are testing filling the 0th position when empty {} and {}",
                    x, 0
                );
            }
            None => {
                panic!("Error the output is empty");
            }
        }
    }

    #[test]
    fn check_player_test() {
        let mut m = [0; 9]; // tic tac toe board

        m[0] = HUMAN;
        m[1] = HUMAN;
        m[2] = HUMAN;
        m[3] = 0;
        m[4] = 0;
        m[5] = 0;
        m[6] = 0;
        m[7] = 0;
        m[8] = 0;

        let mut count: u32 = 0;

        let human_moves: usize = check_player(&m, HUMAN);

        assert_eq!(
            human_moves, 3,
            "we are testing 3 human moves {} and {}",
            human_moves, 0
        );
    }

    #[test]
    fn check_player_test1() {
        let mut m = [0; 9]; // tic tac toe board

        m[0] = 0;
        m[1] = 0;
        m[2] = 0;
        m[3] = HUMAN;
        m[4] = HUMAN;
        m[5] = HUMAN;
        m[6] = 0;
        m[7] = 0;
        m[8] = 0;

        let mut count: u32 = 0;

        let human_moves: usize = check_player(&m, HUMAN);

        assert_eq!(
            human_moves, 3,
            "we are testing 3 human moves {} and {}",
            human_moves, 0
        );
    }

    #[test]
    fn check_player_test2() {
        let mut m = [0; 9]; // tic tac toe board

        m[0] = 0;
        m[1] = 0;
        m[2] = 0;
        m[3] = 0;
        m[4] = 0;
        m[5] = 0;
        m[6] = HUMAN;
        m[7] = HUMAN;
        m[8] = HUMAN;

        let mut count: u32 = 0;

        let human_moves: usize = check_player(&m, HUMAN);

        assert_eq!(
            human_moves, 3,
            "we are testing 3 human moves {} and {}",
            human_moves, 0
        );
    }

    #[test]
    fn check_player_test3() {
        let mut m = [0; 9]; // tic tac toe board

        m[0] = HUMAN;
        m[1] = 0;
        m[2] = 0;
        m[3] = 0;
        m[4] = 0;
        m[5] = 0;
        m[6] = 0;
        m[7] = 0;
        m[8] = 0;

        let mut count: u32 = 0;

        let human_moves: usize = check_player(&m, HUMAN);

        assert_eq!(
            human_moves, 1,
            "we are testing 3 human moves {} and {}",
            human_moves, 1
        );
    }

    #[test]
    fn check_player_test4() {
        let mut m = [0; 9]; // tic tac toe board

        m[0] = 0;
        m[1] = 0;
        m[2] = 0;
        m[3] = 0;
        m[4] = 0;
        m[5] = 0;
        m[6] = 0;
        m[7] = 0;
        m[8] = HUMAN;

        let mut count: u32 = 0;

        let human_moves: usize = check_player(&m, HUMAN);

        assert_eq!(
            human_moves, 1,
            "we are testing 3 human moves {} and {}",
            human_moves, 1
        );
    }

    #[test]
    fn check_player_test5() {
        let mut m = [0; 9]; // tic tac toe board

        m[0] = 0;
        m[1] = 0;
        m[2] = 0;
        m[3] = 0;
        m[4] = 0;
        m[5] = 0;
        m[6] = HUMAN;
        m[7] = 0;
        m[8] = 0;

        let mut count: u32 = 0;

        let human_moves: usize = check_player(&mut m, HUMAN);

        assert_eq!(
            human_moves, 1,
            "we are testing 3 human moves {} and {}",
            human_moves, 0
        );
    }

    #[test]
    fn check_player_test6() {
        let mut m = [0; 9]; // tic tac toe board

        m[0] = 0;
        m[1] = 0;
        m[2] = 0;
        m[3] = 0;
        m[4] = 0;
        m[5] = 0;
        m[6] = 0;
        m[7] = HUMAN;
        m[8] = HUMAN;

        let mut count: u32 = 0;

        let human_moves: usize = check_player(&m, HUMAN);

        assert_eq!(
            human_moves, 2,
            "we are testing 3 human moves {} and {}",
            human_moves, 2
        );
    }

    #[test]
    fn set_ai_next_move_when_2_occupied_test() {
        let mut m = [0; 9]; // tic tac toe board

        m[0] = HUMAN;
        m[1] = HUMAN;
        m[2] = 0;
        m[3] = 0;
        m[4] = 0;
        m[5] = 0;
        m[6] = 0;
        m[7] = 0;
        m[8] = 0;

        set_ai_next_move_when_2_occupied(&mut m, HUMAN);

        assert_eq!(
            m[2], AI,
            "we are testing ai filling when 2 occupied {} and {}",
            m[2], AI
        );
    }

    #[test]
    fn set_ai_next_move_when_2_occupied_test2() {
        let mut m = [0; 9]; // tic tac toe board

        m[0] = 0;
        m[1] = HUMAN;
        m[2] = HUMAN;
        m[3] = 0;
        m[4] = 0;
        m[5] = 0;
        m[6] = 0;
        m[7] = 0;
        m[8] = 0;

        set_ai_next_move_when_2_occupied(&mut m, HUMAN);

        assert_eq!(
            m[0], AI,
            "we are testing ai filling when 2 occupied {} and {}",
            m[0], AI
        );
    }
}
