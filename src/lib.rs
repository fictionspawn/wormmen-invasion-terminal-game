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
}
/*
impl MoveChar {
    const mut x = 0;
    const mut y = 0;
}
*/
pub struct MoveWormman {
    wormman: bool,
    wx1: i32,
    wy1: i32,
    wx1kill: i32,
}

pub struct Item {
    inventory: Vec<String>,
    lantern_picked_up: bool,
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
    };
/*
    let mut x:i32 = 0;
   // let mut y:i32 = 0;
    let mut ladder_up:bool = false;
    let mut ladder_down:bool = false;
    let mut lantern_picked_up:bool = false;
    let mut wormman = false;
    let mut wx1 = 10;
    let wy1 = 1;
    let mut wx1kill:i32;
    let mut inventory:Vec<String> = Vec::new();
*/

    let game:bool = true;

    while game == true {         
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
            println!("Wormmen Invasion is a text based 2D platformer.\nMove your character using wasd. Press i for inventory. Press h for help.\n");
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
            if move_char.x == 5 {
                move_char.ladder_up = true;
            }
            if move_char.x < 1 {
                println!("There's a solid brick wall to your left.");
                move_char.x = 0;
            }
            if move_char.x > 13 {
                if item.inventory.contains(&"Dungeon Key".to_string()) {
                    println!("Congratulations! You escaped the dungeon!");
                    break;
                } else {
                    println!("You reach a locked door.");
                }
                move_char.x = 14;
            }
            if move_char.x == 4 || move_char.x == 6 {
                move_char.ladder_up = false;
            }
            if move_char.x == 10 {
                if item.lantern_picked_up == false {
                    if buffer == [101] {
                        item.inventory.push("Lantern".to_string());
                        item.lantern_picked_up = true;
                        println!("You picked up the lantern");
                } else {
                    println!("There's a body on the ground. He or she, it's hard to tell from all the gore, holds a lantern. Press \"e\" to pick it up.");
                    }
            } else {
                println!("There's a body on the ground. It looks like it has been eaten by something.");
                    }            
            }
        } else if move_char.y == 1 {
            if move_char.x == 5 {
                move_char.ladder_down = true;
                if buffer == [115] {
                    move_char.ladder_up = true;
                }
            }
            if move_char.x == 4 || move_char.x == 6 {
                move_char.ladder_down = false;
            }
            if move_char.x == 1 {
                println!("Stone stairs leads down into darkness.");
                if buffer == [115] {
                    if item.inventory.contains(&"Lantern".to_string()) {
                        println!("You are in a dark cellar, but your lantern guides the path.\nSome weird sounds can be heard from the tunnel to the left.\nTo the right there is a hole in the ground and a rock wall.");
                       move_char.y = -1;
                        move_char.x = 0;
                    } else {
                        println!("It's too dark to go down there.");
                    }
                }
            }
            if move_char.x < -2 {
                println!("You've reached the  Wall");
                move_char.x = -3;
            }
            if move_char.x > 18 {
                println!("You've reached a wall");
                move_char.x = 19;
            }
        } else if move_char.y == -1 {
            if move_char.x == 4 {
                println!("The hole is deep and dark, but small. You can step over or go down.");
                if buffer == [115] {
                    println!("You jump into the hole, and land in the sewers. The stench is unbearable.");
                    move_char.y = -2;
                }
            }
            if move_char.x == 5 {
                println!("There's a key on the ground.");
                if buffer == [101] {
                    item.inventory.push("Dungeon Key".to_string());
                    println!("You picked up the key");
                }
            }
            if move_char.x > 7 {
                println!("You've reached the wall. The natural rock indicates you are under ground.");
                move_char.x = 8;
            }
            if move_char.x == 0 {
                println!("There's a staircase going up.");
                if buffer == [119] {
                    move_char.y = 1;
                    println!("There's a staircase going down.");
                }
            }
            if move_char.x == -6 {
                if last == -5 {
                    println!("The sounds from the darkness are getting closer.");
                }
            }
            if move_char.x == -12 {
                if last == -11 {
                    println!("Something moves in the shadows.");
                }
            }
            if move_char.x == -15 {
                println!("Wormmen fall down from the ceiling, grab you from the darkness, crawl at you on the ground. You die in terror as slimy mouths devour you.");
                break
            }
        }
        if move_char.ladder_up == true {
            if buffer ==[119] {
                move_char.ladder_up = false;
                move_char.ladder_down = true;
                move_char.y += 1;
                println!("There's a disgusting creature with human upper body and worm tail for legs crawling back and forth.");
            } else {
                println!("There's a ladder going up.");
            }
        }
        if move_char.ladder_down == true {
            if buffer == [115] {
                move_char.ladder_down = false;
                move_char.ladder_up = true;
                move_char.y -= 1;
            } else {
                println!("There's a ladder going down.");
            }
        }
        if move_wormman.wx1 < -2 {
           move_wormman.wormman = true;
        }
        if move_wormman.wx1 > 13 {
            move_wormman.wormman = false;
        }
        if move_wormman.wormman {
            move_wormman.wx1 += 1;
            move_wormman.wx1kill = move_wormman.wx1 + 1;
        } else {
            move_wormman.wx1 -= 1;
            move_wormman.wx1kill = move_wormman.wx1 - 1;
        }
        if move_wormman.wx1 == move_char.x || move_wormman.wx1kill == move_char.x {
            if move_wormman.wy1 == move_char.y {
                println!("The wormman eats you alive as you scream in terror.");
                break
            }
        }
        if buffer == [120] {
            break
        }
        println!("{} , {}", move_char.x, move_char.y);
        if move_char.y == 1 || (move_char.x == 5 && move_char.y == 0) {
            println!("Wormman: {}, {}", move_wormman.wx1, move_wormman.wy1);
        }
    }
}


