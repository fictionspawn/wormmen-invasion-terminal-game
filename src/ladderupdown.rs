use crate::GameState;

pub fn ladder_up_down(state: &mut GameState, buffer: [u8; 1]) {
    if state.move_char.ladder_up == true {
        if buffer ==[119] {
            state.move_char.ladder_up = false;
            state.move_char.ladder_down = true;
            state.move_char.y += 1;
            if state.move_wormman.wormman {
                println!("There's a disgusting creature with human upper body and worm tail for legs crawling back and forth.");
            }
        } else {
            println!("There's a ladder going up.");
        }
    }
    if state.move_char.ladder_down == true {
        if buffer == [115] {
            state.move_char.ladder_down = false;
            state.move_char.ladder_up = true;
            state.move_char.y -= 1;
        } else {
            println!("There's a ladder going down.");
        }
    }
}

pub fn climb_lamp(state: &mut GameState, buffer: [u8;1]) {
    if buffer == [115] {
        println!("You drop yourself back down on the stone floor.");
        state.move_char.y = 1; 
                
    }
    if buffer ==[119] {
        let something: &str;
        if state.item.blade_taken == false {
            something = "\nA man sits against the wall by a window by the wall to your left."
        } else { 
            something = "";
        }
    println!("You are in some kind of attic. Some crates and rubbish lie around. Moonlight from a window in the ceiling lights up the room.{}", something);
    state.move_char.y = 3;
    }
    if state.move_char.x < 10 || state.move_char.x > 10 {
                state.move_char.x = 10;
    }
}

pub fn staircase_down(state: &mut GameState) {
        if state.item.inventory.contains(&"Lantern".to_string()) {
            println!("You walk down winding stone stairs. The light from your lantern flickers on the walls.\nStrange sounds are coming from the tunnel to the left. To your right there's a hole in the ground, and a wall further ahead.");
            state.move_char.y = -1;
            state.move_char.x = 2;
        } else {
            println!("It's too dark to go down there.");
    }
}

pub fn staircase_up(state: &mut GameState) {
    state.move_char.y = 1;
    state.move_char.x = 0;
    println!("You walk back up the stairs.");
}
