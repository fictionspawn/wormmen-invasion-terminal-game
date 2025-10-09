use crate::{GameState};
use crate::ladderupdown::staircase_up;

pub fn basement(state: &mut GameState, buffer: [u8; 1], last: i32) {
    if state.move_char.x == 4 {
        println!("The hole is deep and dark, but small.\nYou can step over or go down.");
        if buffer == [115] {
            println!("You jump into the hole, and land in the sewers.\nThe stench is unbearable, making it hard to breath.");
            state.move_char.y = -2;
        } 
    }
    else if state.move_char.x == 5 {
        if !state.item.key_picked_up { 
            println!("There's a key on the ground.");
            if buffer == [101] {
                state.item.inventory.push("Dungeon Key".to_string());
                println!("You picked up the key");
                state.item.key_picked_up = true;
            } 
        } 
    }
    else if state.move_char.x > 7 {
        println!("You've reached the wall. The natural rock indicates you are under ground.");
        state.move_char.x = 8;
    }
    else if state.move_char.x == 2 {
        let something: &str;
        if state.move_wormman.x < 5 && state.move_wormman.wormman {
            something = "You hear the wormman crawl around up there.";
        } else { 
            something = "";
        }
        if buffer == [119] {
            staircase_up(state);
        } else {    
        println!("There's a staircase going up. {}", something);
        }
    }
    else if state.move_char.x == -6 {
        if last == -5 {
            println!("The sounds from the darkness are getting closer.");
        }
    }
    else if state.move_char.x == -12 {
        if last == -11 {
            println!("Something moves in the shadows.");
        } 
    }
    else if state.move_char.x == -15 {
        println!("Wormmen fall down from the ceiling, grab you from the darkness, crawl at you on the ground.\nYou die in terror as slimy mouths devour you.");
        state.death = true;
    }
}

pub fn sewers(state: &mut GameState) {
    state.move_char.sewer_stepcount += 1;
    if state.move_char.sewer_stepcount == 10 {
        println!("The gases from the stinking water are making you dizzy.")
    }
    else if state.move_char.sewer_stepcount == 15 {
        println!("You stumble in the dirty water. Gasping for air, you swallow a lot.");
    }
    else if state.move_char.sewer_stepcount > 17 {
        println!("The smell is taking over. The tunnel starts spinning faster.\nYou fall face down in the sewers with no one to save you.");
            state.death = true;
    } 
}
