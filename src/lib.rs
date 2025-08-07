mod groundfloor;
use crate::groundfloor::ground_floor;
mod firstfloor;
use crate::firstfloor::{first_floor, climb_lamp, attic};
mod basement;
use crate::basement::{basement, sewers};
mod wormman;
use crate::wormman::wormman_move;
mod ladderupdown;
use crate::ladderupdown::ladder_up_down;

extern crate termios;
use std::io;
use std::io::Read;
use std::io::Write;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};


pub struct MoveChar {
    x: i32,
    y: i32,   
    ladder_up: bool,
    ladder_down: bool,
    sewer_stepcount: u32,
    step_count: u32,
}

pub struct MoveWormman {
    wormman: bool,
    wx1: i32,
    wy1: i32,
    wx1kill: i32,
}

pub struct WindowWormman {
    wormman: bool,
    x: i32,
    y: i32,
    wwkill: i32,
}

pub struct Item {
    inventory: Vec<String>,
    lantern_picked_up: bool,
    key_picked_up: bool,
    blade_taken: bool,
}

pub struct GameState {
    pub move_char: MoveChar,
    pub item: Item,
    pub death: bool,
    pub move_wormman: MoveWormman,
    pub window_wormman: WindowWormman,
} 

impl GameState {
    pub fn new() -> Self {
        GameState {
            move_char: MoveChar {
                x: 0,
                y: 0,
                ladder_up: false,
                ladder_down: false,
                sewer_stepcount: 0,
                step_count: 0,
            },
            item: Item {
                inventory: Vec::new(),
                lantern_picked_up: false, 
                key_picked_up: false,
                blade_taken: false,
            },
            death: false,
            move_wormman: MoveWormman {
                wormman: false,
                wx1: 10,
                wy1: 1,
                wx1kill: 11,
            },
            window_wormman: WindowWormman {
                wormman: false,
                x: 12,
                y: 3,
                wwkill: 11,
            },
            //last_x: 0,
        }
    }
}


pub fn intro() { 
    println!("\nWelcome to WormmenInvasion! \n\nYou are at the ground floor. There's a brick wall to your left. \nThere's a ladder in sight, and a dim glow beyond the ladder. \n\nMove your character with wasd. Press i for inventory. Press h for help, x to exit the game.\n");
}

pub fn play_game() {
    let mut state = GameState::new();

    let game:bool = true;
  //  let mut death:bool = false;

    while game {   

        //Game contol, keydown movement
        let stdin = 0;
        let termios = Termios::from_fd(stdin).unwrap();
        let mut new_termios = termios.clone();
        new_termios.c_lflag &= !(ICANON | ECHO);
        tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
        let stdout = io::stdout();
        let mut reader = io::stdin();
        let mut buffer = [0;1];
        stdout.lock().flush().unwrap();
        reader.read_exact(&mut buffer).unwrap();
        tcsetattr(stdin, TCSANOW, & termios).unwrap();

        let last = state.move_char.x;
        state.move_char.step_count += 1;

        if buffer == [104] {
            println!("Wormmen Invasion is a text based 2D platformer.\nMove your character using wasd. Press i for inventory.\nPress h for help. Press x to exit the game.");
        }
        if buffer == ([105]) {
            println!("Inventory: {:?}", state.item.inventory);
        }
        if buffer == [97] {
            state.move_char.x -= 1;
        }
        if buffer == [100] {
            state.move_char.x += 1;
        }
        if state.move_char.y == 0 {
            ground_floor(&mut state, buffer);
        } else if state.move_char.y == 1 {
            first_floor(&mut state, buffer);
        } else if state.move_char.y == -1 {
            basement(&mut state, buffer, last);
        } else if state.move_char.y == 2 {
           climb_lamp(&mut state, buffer);
        } else if state.move_char.y == -2 {
            sewers(&mut state);
        } else if state.move_char.y == 3 {
            attic(&mut state, buffer);
        }
        ladder_up_down(&mut state, buffer);
        wormman_move(&mut state);

        println!("Step count: {}", state.move_char.step_count);

        if state.death {
            break;
        }
        if buffer == [120] {
            break
        }
        println!("{} , {}", state.move_char.x, state.move_char.y);
        if state.move_char.y == 1 || (state.move_char.x == 5 && state.move_char.y == 0) || state.move_char.y == 2  || (state.move_char.y == 3 && state.move_char.x == 10) {
            println!("Wormman: {}, {}", state.move_wormman.wx1, state.move_wormman.wy1);
        }
    }
}


