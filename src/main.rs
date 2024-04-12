use std::{fmt::Debug, io};

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
    Init,
    MainMenu,
    GameOver,
}

struct Game {
    state: GameState,
    field: [CellValue; 9]
}

impl Game {
    fn new_game() -> Self {
        Self {
            state: GameState::Init,
            field: [CellValue::Empty; 9]
        }
    }

    fn get_main_menu() -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.push(String::from("==================="));
        vec.push(String::from("= Крестики-нолики ="));
        vec.push(String::from("=     на Rust     ="));
        vec.push(String::from("==================="));
        vec.push(String::from(""));
        vec.push(String::from("1. Играть против компьютера"));
        vec.push(String::from("2. Играть вдвоём"));
        vec.push(String::from("3. Выход"));
        vec.push(String::from(""));
        vec
    }

    fn get_game_over() -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.push(String::from("Спасибо за игру!"));
        vec
    }
}

impl Game {
    fn is_game_over(&self) -> bool {
        if let GameState::GameOver = self.state { true } else { false }
    }

    fn request(&mut self, req: Option<String>) -> Vec<String> {
        match self.state {
            GameState::Init => {
                self.state = GameState::MainMenu;
                Self::get_main_menu()
            }
            GameState::MainMenu => {
                self.check_main_menu(req)
            },
            GameState::GameOver => Self::get_game_over()
        }
    }

    fn check_main_menu(&mut self, st: Option<String>) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        match st {
            Some(st) => {
                match st.trim().parse() {
                    Ok(n) => {
                        let choice: u8 = n;
                        match choice {
                            1 => vec,
                            2 => vec,
                            3 => {
                                self.state = GameState::GameOver;
                                return Self::get_game_over();
                            }
                            _ => {
                                vec.push(String::from("Введи номер пункта меню!"));
                                return vec
                            }
                        }
                    }
                    Err(_) => {
                        vec.push(String::from("Введи номер пункта меню!"));
                        return vec
                    }
                }
            }
            None => {
                vec.push(String::from("Введи номер пункта меню!"));
                return vec
            }
        }
    }

    fn get_field(&self) -> Vec<String> {
        // ===========
        // =  _|_|_  =
        // =  _|_|_  =
        // =   | |   =
        // ===========
        let mut vec: Vec<String> = Vec::new();

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

        vec.push(String::from("==========="));
        vec.push(line_1);
        vec.push(line_2);
        vec.push(line_3);
        vec.push(String::from("==========="));
        vec
    }
}

fn show_vec(vec: Vec<String>) {
    for st in vec {
        println!("{st}");
    }
}

fn main() {
    let mut game: Game = Game::new_game();

    let vec: Vec<String> = game.request(None);
    show_vec(vec);

    while !game.is_game_over() {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Не удалось прочитать строку");

        let vec: Vec<String> = game.request(Some(input));
        show_vec(vec);
    }
}