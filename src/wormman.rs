use crate::{MoveWormman, MoveChar};

pub fn wormman_move(move_wormman: &mut MoveWormman, move_char: &mut MoveChar, death: &mut bool) {
    if move_wormman.wx1 < -3 {
        move_wormman.wormman = true;
  //      move_wormman.wx1kill = move_wormman.wx1 + 1;
    }
    if move_wormman.wx1 > 14 {
        move_wormman.wormman = false;
    //    move_wormman.wx1kill = move_wormman.wx1 - 1;
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
               *death = true;
        }
    }
}


