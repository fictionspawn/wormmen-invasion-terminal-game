use crate::{GameState};

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
    if state.move_char.x == 1 {
        println!("Stone stairs leads down into darkness.");
        if buffer == [115] {
            if state.item.inventory.contains(&"Lantern".to_string()) {
                println!("You are in a dark cellar, but your lantern guides the path.\nSome weird sounds can be heard from the tunnel to the left.\nTo the right there is a hole in the ground and a rock wall.");
                state.move_char.y = -1;
                state.move_char.x = 0;
            } else {
                println!("It's too dark to go down there.");
            }
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
            println!("You step on the table,jump up and hang on to the lamp.\nThe wormman crawls around on the floor below. He seems to have forgotten all about you.");
            state.move_char.y = 2; 
       } else {
        println!("There's a table section with some chairs.\nA lamp is hanging in a chain from the ceiling.");
        }
    }
}

pub fn climb_lamp(state: &mut GameState, buffer: [u8;1]) {
    if buffer == [115] {
        println!("You drop yourself back down on the stone floor.");
        state.move_char.y = 1; 
                
    }
    if buffer ==[119] {
    println!("You are in some kind of attic. Some crates and rubbish lie around.\nA man sits against the wall by a window by the wall to your left.");
    state.move_char.y = 3;
    }
    if state.move_char.x < 10 || state.move_char.x > 10 {
                state.move_char.x = 10;
    }
}

pub fn attic(state: &mut GameState, buffer: [u8; 1]) {
    if state.move_char.x == 10 {
        if buffer == [115] {
            state.move_char.y = 2;
        } else {
        println!("There's a chain lamp going down to the floor below. A wormman is crawling around down there.");
        }
    } else if state.move_char.x <= 5 {
        if buffer == [101] && !state.item.blade_taken {
            println!("You touch the man's shoulder, to see if he needs help.\nHe coughs. \"They... they are everywhere. Be care...ful.\"\nYou try to comfort him, but his wounds tells their own tale.\n\"Here, take this. It will protect you.\"\nHe holds out a knife, a dagger with a dark blade. \"It's magi...*\"\nHe dies. You take the blade.");
            state.item.blade_taken = true;
        } else if !state.item.blade_taken {
            println!("The man stares into nothingness. He seems to be alive.");
            state.move_char.x = 5;
        } else {
            println!("What a horrible fate...");
    }
    }
    if state.move_char.x == 7 && state.item.blade_taken && !state.window_wormman.wormman {
        println!("A wormman falls in through the window and comes crawling towards you.");
        state.window_wormman.wormman = true;
        state.window_wormman.x = 12;
        state.window_wormman.y = 3;
        }
    if state.window_wormman.wormman {
        state.window_wormman.x -= 1;
        state.window_wormman.wwkill = state.window_wormman.x - 1;
        println!("Wormman: {}, {}", state.window_wormman.x, state.window_wormman.y);
        if state.window_wormman.x == state.move_char.x || state.window_wormman.wwkill == state.move_char.x {
            state.death = true;
            println!("The wormmman devours you. You die in horror.");
        }
    }
}
