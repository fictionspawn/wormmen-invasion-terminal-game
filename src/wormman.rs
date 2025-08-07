use crate::{GameState};

pub fn wormman_move(state: &mut GameState) {
    if state.move_wormman.wx1 < -3 {
        state.move_wormman.wormman = true;
  //      move_wormman.wx1kill = move_wormman.wx1 + 1;
    }
    if state.move_wormman.wx1 > 14 {
        state.move_wormman.wormman = false;
    //    move_wormman.wx1kill = move_wormman.wx1 - 1;
    }
    if state.move_wormman.wormman {
        state.move_wormman.wx1 += 1;
        state.move_wormman.wx1kill = state.move_wormman.wx1 + 1;
    } else {
        state.move_wormman.wx1 -= 1;
        state.move_wormman.wx1kill = state.move_wormman.wx1 - 1;
    }
    if state.move_wormman.wx1 == state.move_char.x || state.move_wormman.wx1kill == state.move_char.x {
        if state.move_wormman.wy1 == state.move_char.y {
                println!("The wormman eats you alive as you scream in terror.");
               state.death = true;
        }
    }
}


