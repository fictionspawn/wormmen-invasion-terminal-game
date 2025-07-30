mod groundfloor;
use crate::groundfloor::ground_floor;
mod firstfloor;
use crate::firstfloor::{first_floor, climb_lamp};
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
    sewer_stepcount: i32,
}

pub struct MoveWormman {
    wormman: bool,
    wx1: i32,
    wy1: i32,
    wx1kill: i32,
}

pub struct Item {
    inventory: Vec<String>,
    lantern_picked_up: bool,
    key_picked_up: bool,
}



pub fn intro() { 
    println!("\nWelcome to WormmenInvasion! \n\nYou are at the ground floor. There's a brick wall to your left. \nThere's a ladder in sight, and a dim glow beyond the ladder. \n\nMove your character with wasd. Press i for inventory. Press h for help.\n");
}

pub fn play_game() {
    let mut move_char = MoveChar {
        x: 0,
        y: 0,
        ladder_up: false,
        ladder_down: false,
        sewer_stepcount: 0,
    };
    let mut move_wormman = MoveWormman {
        wormman: false,
        wx1: 10,
        wy1: 1,
        wx1kill: 11, 
    };
    let mut item = Item {
        inventory: Vec::new(),
        lantern_picked_up: false,
        key_picked_up: false,
    };

    let game:bool = true;
    let mut death:bool = false;

    while game == true {   

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

        let last = move_char.x;

        if buffer == [104] {
            println!("Wormmen Invasion is a text based 2D platformer.\nMove your character using wasd. Press i for inventory.\nPress h for help. Press x to exit the game.");
        }
        if buffer == ([105]) {
            println!("Inventory: {:?}", item.inventory);
        }
        if buffer == [97] {
            move_char.x -= 1;
        }
        if buffer == [100] {
            move_char.x += 1;
        }
        if move_char.y == 0 {
            ground_floor(&mut move_char, &mut item, buffer, &mut death);
        } else if move_char.y == 1 {
            first_floor(&mut move_char, &mut item, buffer);
        } else if move_char.y == -1 {
            basement(&mut move_char, &mut item, buffer, &mut death, last);
        } else if move_char.y == 2 {
           climb_lamp(&mut move_char, buffer);
        } else if move_char.y == -2 {
            sewers(&mut move_char, &mut death);
        }
        ladder_up_down(&mut move_char, buffer);
        wormman_move(&mut move_wormman, &mut move_char, &mut death);
        if death {
            break;
        }
        if buffer == [120] {
            break
        }
        println!("{} , {}", move_char.x, move_char.y);
        if move_char.y == 1 || (move_char.x == 5 && move_char.y == 0) || move_char.y == 2 {
            println!("Wormman: {}, {}", move_wormman.wx1, move_wormman.wy1);
        }
    }
}


