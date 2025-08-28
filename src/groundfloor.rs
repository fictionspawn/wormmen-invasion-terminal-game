use crate::{GameState};

pub fn ground_floor(state: &mut GameState, buffer: [u8; 1]) {
    if state.move_char.x == 5 {
        state.move_char.ladder_up = true;
    }
    if state.move_char.x < 1 {
        println!("There's a solid brick wall to your left.");
        state.move_char.x = 0;
    }
    if state.move_char.x > 13 {
        if state.item.inventory.contains(&"Dungeon Key".to_string()) {
            println!("Congratulations! You escaped the dungeon!");
            state.death = true;
        } else {
            println!("You reach a locked door.");
        }
        state.move_char.x = 14;
    }
    if state.move_char.x == 4 || state.move_char.x == 6 {
        state.move_char.ladder_up = false;
    }
    if state.move_char.x == 10 {
        if state.item.lantern_picked_up == false {
            if buffer == [101] {
                state.item.inventory.push("Lantern".to_string());
                state.item.lantern_picked_up = true;
                println!("You picked up the lantern");
            } else {
                println!("There's a body on the ground. He or she, it's hard to tell from all the gore, holds a lantern.\nPress \"e\" to pick it up.");
            }
        } else {
            println!("There's a body on the ground. It looks like it has been eaten by something.");
        }            
    }
}

