use crate::{MoveChar, Item};

pub fn ground_floor(move_char: &mut MoveChar, item: &mut Item, buffer: [u8; 1], death: &mut bool) {
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
            *death = true;
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
                println!("There's a body on the ground. He or she, it's hard to tell from all the gore, holds a lantern.\nPress \"e\" to pick it up.");
            }
        } else {
            println!("There's a body on the ground. It looks like it has been eaten by something.");
        }            
    }
}

