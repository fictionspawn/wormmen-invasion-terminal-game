use crate::GameState;
use crate::ladderupdown::staircase_down;

pub fn first_floor(state: &mut GameState, buffer: [u8; 1]) {
    if state.move_char.x == 5 {
        state.move_char.ladder_down = true;
        if buffer == [115] {
            state.move_char.ladder_up = true;
        }
    }
    if state.move_char.x == 4 || state.move_char.x == 6 {
        state.move_char.ladder_down = false;
    }
    if state.move_char.x == 0 {
        println!("Stone stairs lead down into darkness.");
        if buffer == [115] {
            staircase_down(state);
          /*  if state.item.inventory.contains(&"Lantern".to_string()) {
                println!("You are in a dark cellar, but your lantern guides the path.\nSome weird sounds can be heard from the tunnel to the left.\nTo the right there is a hole in the ground and a rock wall.");
                state.move_char.y = -1;
                state.move_char.x = 2;
            } else {
                println!("It's too dark to go down there.");
            }*/
        }
    }
    if state.move_char.x < -2 {
        println!("You've reached the  Wall");
        state.move_char.x = -3;
    }
    if state.move_char.x > 13 {
        println!("You've reached a wall");
        state.move_char.x = 14;
    }
    if state.move_char.x == 10 {
         if buffer == [119] {
             let something: &str;
             if state.move_wormman.wormman {
                something = "\nThe wormman crawls around on the floor below. He seems to have forgotten all about you."
             } else {
                something = "";
             }
            println!("You step on the table,jump up and hang on to the lamp.{}", something);
            state.move_char.y = 2; 
       } else {
        println!("There's a table section with some chairs.\nA lamp is hanging in a chain from the ceiling.");
        }
    }
}

