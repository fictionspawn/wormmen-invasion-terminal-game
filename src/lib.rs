extern crate termios;
use std::io;
use std::io::Read;
use std::io::Write;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

pub fn intro() { 
    println!("\nWelcome to WormmenInvasion! \n\nYou are at the ground floor. There's a brick wall to your left. \nThere's a ladder in sight, and a dim glow beyond the ladder. \n\nMove your character with wasd. Press i for inventory. Press h for help.\n");
}

pub fn play_game() {
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let game:bool = true;
    let mut ladder_up:bool = false;
    let mut ladder_down:bool = false;
    let mut lantern_picked_up:bool = false;
    let mut wormman = false;
    let mut wx1 = 10;
    let wy1 = 1;
    let mut wx1kill:i32;
    let mut inventory:Vec<String> = Vec::new();


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

        let last = x;

        if buffer == [104] {
            println!("Wormmen Invasion is a text based 2D platformer.\nMove your character using wasd. Press i for inventory. Press h for help.\n");
        }
        if buffer == ([105]) {
            println!("Inventory: {:?}", inventory);
        }
        if buffer == [97] {
            x -= 1;
        }
        if buffer == [100] {
            x += 1;
        }
        
        if y == 0 {
            if x == 5 {
                ladder_up = true;
            }
            if x < 1 {
                println!("There's a solid brick wall to your left.");
                x = 0;
            }
            if x > 13 {
                if inventory.contains(&"Dungeon Key".to_string()) {
                    println!("Congratulations! You escaped the dungeon!");
                    break;
                } else {
                    println!("You reach a locked door.");
                }
                x = 14;
            }
            if x == 4 || x == 6 {
                ladder_up = false;
            }
            if x == 10 {
                if lantern_picked_up == false {
                    if buffer == [101] {
                        inventory.push("Lantern".to_string());
                        lantern_picked_up = true;
                        println!("You picked up the lantern");
                } else {
                    println!("There's a body on the ground. He or she, it's hard to tell from all the gore, holds a lantern. Press \"e\" to pick it up.");
                    }
            } else {
                println!("There's a body on the ground. It looks like it has been eaten by something.");
                    }            
            }
        } else if y == 1 {
            if x == 5 {
                ladder_down = true;
                if buffer == [115] {
                    ladder_up = true;
                }
            }
            if x == 4 || x == 6 {
                ladder_down = false;
            }
            if x == 1 {
                println!("Stone stairs leads down into darkness.");
                if buffer == [115] {
                    if inventory.contains(&"Lantern".to_string()) {
                        println!("You are in a dark cellar, but your lantern guides the path.\nSome weird sounds can be heard from the tunnel to the left.\nTo the right there is a hole in the ground and a rock wall.");
                        y = -1;
                        x = 0;
                    } else {
                        println!("It's too dark to go down there.");
                    }
                }
            }
            if x < -2 {
                println!("You've reached the  Wall");
                x = -3;
            }
            if x > 18 {
                println!("You've reached a wall");
                x = 19;
            }
        } else if y == -1 {
            if x == 4 {
                println!("The hole is deep and dark, but small. You can step over or go down.");
                if buffer == [115] {
                    println!("You jump into the hole, and land in the sewers. The stench is unbearable.");
                    y = -2;
                }
            }
            if x == 5 {
                println!("There's a key on the ground.");
                if buffer == [101] {
                    inventory.push("Dungeon Key".to_string());
                    println!("You picked up the key");
                }
            }
            if x > 7 {
                println!("You've reached the wall. The natural rock indicates you are under ground.");
                x = 8;
            }
            if x == 0 {
                println!("There's a staircase going up.");
                if buffer == [119] {
                    y = 1;
                    println!("There's a staircase going down.");
                }
            }
            if x == -6 {
                if last == -5 {
                    println!("The sounds from the darkness are getting closer.");
                }
            }
            if x == -12 {
                if last == -11 {
                    println!("Something moves in the shadows.");
                }
            }
            if x == -15 {
                println!("Wormmen fall down from the ceiling, grab you from the darkness, crawl at you on the ground. You die in terror as slimy mouths devour you.");
                break
            }
        }
        if ladder_up == true {
            if buffer ==[119] {
                ladder_up = false;
                ladder_down = true;
                y += 1;
                println!("There's a disgusting creature with human upper body and worm tail for legs crawling back and forth.");
            } else {
                println!("There's a ladder going up.");
            }
        }
        if ladder_down == true {
            if buffer == [115] {
                ladder_down = false;
                ladder_up = true;
                y -= 1;
            } else {
                println!("There's a ladder going down.");
            }
        }
        if wx1 < -2 {
            wormman = true;
        }
        if wx1 > 13 {
            wormman = false;
        }
        if wormman {
            wx1 += 1;
            wx1kill = wx1 + 1;
        } else {
            wx1 -= 1;
            wx1kill = wx1 - 1;
        }
        if wx1 == x || wx1kill == x {
            if wy1 == y {
                println!("The wormman eats you alive as you scream in terror.");
                break
            }
        }
        if buffer == [120] {
            break
        }
        println!("{} , {}", x, y);
        if y == 1 || (x == 5 && y == 0) {
            println!("Wormman: {}, {}", wx1, wy1);
        }
    }
}


