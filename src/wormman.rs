use crate::{GameState};

pub fn wormman_move(state: &mut GameState, buffer: [u8; 1]) {
    if state.move_wormman.wormman == true {
        if state.move_wormman.x < -3 {
            state.move_wormman.wormman_right = true;
        }
        if state.move_wormman.x > 14 {
            state.move_wormman.wormman_right = false;
        }
        if state.move_wormman.wormman_right {
            state.move_wormman.x += 1;
            state.move_wormman.wkill = state.move_wormman.x + 1;
        } else {
            state.move_wormman.x -= 1;
            state.move_wormman.wkill = state.move_wormman.x - 1;
        }
            if state.move_wormman.y == state.move_char.y {
        if state.move_wormman.x == state.move_char.x || state.move_wormman.x + 1 == state.move_char.x || state.move_char.x == state.move_wormman.x - 1 { 
                println!("The wormman eats you alive as you scream in terror.");
                state.death = true;
            }
        } 
        if state.item.inventory.contains(&"Blade".to_string()) {
            if ((state.move_wormman.wkill == state.move_char.x - 1 && state.move_wormman.wormman_right) || (state.move_wormman.wkill == state.move_char.x + 1 && state.move_wormman.wormman_right == false)) && state.move_wormman.wormman && state.move_char.y == 1 {
            if buffer == [101] {
                println!("You stab the wormman in the chest. Dark red blood pumps out. The wormman falls over, gasping for air. It dies.");
                state.move_wormman.wormman = false;
            }
            }
        }
    }
}

pub fn window_wormman(state: &mut GameState, buffer: [u8; 1]) {
    if state.move_char.y == 3 {
    if state.move_char.x == 7 && state.item.blade_taken && !state.window_wormman.wormman && !state.window_wormman.wormman_init { 
        state.window_wormman.wormman_init = true;
        println!("A wormman falls through the wondow and comes crawling towards you.");
        state.window_wormman.wormman = true;
        state.window_wormman.x = 12;
        state.window_wormman.y = 3;
    } else if state.window_wormman.wormman { 
        state.window_wormman.x -= 1;
        state.window_wormman.wkill = state.window_wormman.x - 1;
        println!("Wormman: {}, {}", state.window_wormman.x, state.window_wormman.y);
     
        if (state.window_wormman.x == state.move_char.x || state.window_wormman.x + 1 == state.move_char.x || state.window_wormman.x - 1 == state.move_char.x) && state.window_wormman.wormman {
            state.death = true;
            println!("The wormman devours you. You die in horror.");
        } 
            if state.window_wormman.wkill == state.move_char.x + 1 && state.window_wormman.wormman && state.move_char.y == 3 {
                if buffer == [101] {
                    println!("You stab the wormman in the chest. Dark red blood pumps out. The wormman falls over, gasping for air. It dies.");
                    state.window_wormman.wormman = false;
                }
            }
        }
    }
}

    

