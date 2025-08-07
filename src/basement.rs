use crate::{MoveChar, Item};

pub fn basement(move_char: &mut MoveChar, item: &mut Item, buffer: [u8; 1], death: &mut bool, last: i32) {
    if move_char.x == 4 {
        println!("The hole is deep and dark, but small.\nYou can step over or go down.");
        if buffer == [115] {
            println!("You jump into the hole, and land in the sewers.\nThe stench is unbearable, making it hard to breath.");
            move_char.y = -2;
        }
    }
    if move_char.x == 5 {
        if !item.key_picked_up { 
            println!("There's a key on the ground.");
            if buffer == [101] {
                item.inventory.push("Dungeon Key".to_string());
                println!("You picked up the key");
                item.key_picked_up = true;
            }
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
            move_char.x = 1;
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
        println!("Wormmen fall down from the ceiling, grab you from the darkness, crawl at you on the ground.\nYou die in terror as slimy mouths devour you.");
        *death = true;
    }
}

pub fn sewers(move_char: &mut MoveChar, death: &mut bool) {
    move_char.sewer_stepcount += 1;
    if move_char.sewer_stepcount == 10 {
        println!("The gases from the stinking water are making you dizzy.")
    }
    if move_char.sewer_stepcount == 15 {
        println!("You stumble in the dirty water. Gasping for air, you swallow a lot.");
    }
    if move_char.sewer_stepcount > 17 {
        println!("The smell is taking over. The tunnel starts spinning faster.\nYou fall face down in the sewers with no one to save you.");
            *death = true;
    }
}
