use std::{fmt::Debug, io};
use rand::Rng;

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
enum CellValue {
    Empty,
    X,
    O,
}

enum GameState {
    MainMenu,
    MatchWithBot(MatchWithBot),
    GameOver,
}

enum MatchWithBot {
    PlayerTurn,
    BotTurn,
}

struct Game {
    state: GameState,
    field: [CellValue; 9],
    str_to_show: Vec<String>,
}

impl Game {
    fn new_game() -> Self {
        let mut game = Self {
            state: GameState::MainMenu,
            field: [CellValue::Empty; 9],
            str_to_show: Vec::new(),
        };
        game.write_main_menu();
        game
    }

    fn rand_player() -> u8 {
        let mut rng = rand::thread_rng();

        rng.gen_range(1..3)
    }
}

impl Game {
    fn is_game_over(&self) -> bool {
        if let GameState::GameOver = self.state { true } else { false }
    }

    fn request(&mut self, req: Option<String>) {
        self.str_to_show.clear();

        match &self.state {
            GameState::MainMenu => {
                self.check_main_menu(req);
            },
            GameState::GameOver => self.write_game_over(),
            GameState::MatchWithBot(Turn) => ()
        }
    }

    fn check_main_menu(&mut self, st: Option<String>) {
        match st {
            Some(st) => {
                match st.trim().parse() {
                    Ok(n) => {
                        let choice: u8 = n;
                        match choice {
                            1 => (),
                            2 => (),
                            3 => {
                                self.state = GameState::GameOver;
                                self.write_game_over();
                            }
                            _ => {
                                self.str_to_show.push(String::from("Введи номер пункта меню!"));
                            }
                        }
                    }
                    Err(_) => {
                        self.str_to_show.push(String::from("Введи номер пункта меню!"));
                    }
                }
            }
            None => {
                self.str_to_show.push(String::from("Введи номер пункта меню!"));
            }
        }
    }
    
    fn write_main_menu(&mut self) {
        self.str_to_show.push(String::from("==================="));
        self.str_to_show.push(String::from("= Крестики-нолики ="));
        self.str_to_show.push(String::from("=     на Rust     ="));
        self.str_to_show.push(String::from("==================="));
        self.str_to_show.push(String::from(""));
        self.str_to_show.push(String::from("1. Играть против компьютера"));
        self.str_to_show.push(String::from("2. Играть вдвоём"));
        self.str_to_show.push(String::from("3. Выход"));
        self.str_to_show.push(String::from(""));
    }

    fn write_game_over(&mut self) {
        self.str_to_show.push(String::from("Спасибо за игру!"));
    }

    fn get_field(&mut self) {
        // ===========
        // =  _|_|_  =
        // =  _|_|_  =
        // =   | |   =
        // ===========

        let mut counter: u8 = 0;
        let mut cell_value: &str;
        let mut delim: &str;

        let mut line_1: String = String::from("=  ");
        let mut line_2: String = String::from("=  ");
        let mut line_3: String = String::from("=  ");

        // Пробегаем по полю, формируем три строки для отображения
        for i in &self.field {
            // Определяем, какой символ вставить в ячейку
            match i {
                CellValue::Empty => {
                    if counter < 6 {
                        cell_value = "_";
                    } else {
                        cell_value = " ";
                    }
                }
                CellValue::O => {
                    cell_value = "O";
                }
                CellValue::X => {
                    cell_value = "X";
                }
            }
            // Определяем, чем отделить ячейку от следующего символа
            if counter == 2 || counter == 5 || counter == 8 {
                delim = "";
            } else {
                delim = "|";
            }
            // Определяем в какую строку отправить результат
            if counter <= 2 {
                line_1.push_str(cell_value);
                line_1.push_str(delim);
            } else if counter <= 5 {
                line_2.push_str(cell_value);
                line_2.push_str(delim);
            } else {
                line_3.push_str(cell_value);
                line_3.push_str(delim);
            }

            counter = counter + 1;
        }
        
        line_1.push_str("  =");
        line_2.push_str("  =");
        line_3.push_str("  =");

        self.str_to_show.push(String::from("==========="));
        self.str_to_show.push(line_1);
        self.str_to_show.push(line_2);
        self.str_to_show.push(line_3);
        self.str_to_show.push(String::from("==========="));
    }

    fn show(&mut self) {
        for st in &self.str_to_show {
            println!("{st}");
        }
    }
}

fn main() {
    let mut game: Game = Game::new_game();
    game.show();

    while !game.is_game_over() {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Не удалось прочитать строку");

        game.request(Some(input));
        game.show();
    }
}