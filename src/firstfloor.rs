use crate::{MoveChar, Item};

pub fn first_floor(move_char: &mut MoveChar, item: &mut Item, buffer: [u8; 1]) {
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
    if move_char.x > 13 {
        println!("You've reached a wall");
        move_char.x = 14;
    }
    if move_char.x == 10 {
         if buffer == [119] {
            println!("You step on the table,jump up and hang on to the lamp.\nThe wormman crawls around on the floor below. He seems to have forgotten all about you.");
            move_char.y = 2; 
       } else {
        println!("There's a table section with some chairs.\nA lamp is hanging in a chain from the ceiling.");
        }
    }
}

pub fn climb_lamp(move_char: &mut MoveChar, buffer: [u8;1]) {
    if buffer == [115] {
        println!("You drop yourself back down on the stone floor.");
        move_char.y = 1; 
                
    }
    if move_char.x < 10 || move_char.x > 10 {
                move_char.x = 10;
    }
}
