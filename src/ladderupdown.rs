use crate::GameState;

pub fn ladder_up_down(state: &mut GameState, buffer: [u8; 1]) {
    if state.move_char.ladder_up == true {
        if buffer ==[119] {
            state.move_char.ladder_up = false;
            state.move_char.ladder_down = true;
            state.move_char.y += 1;
            println!("There's a disgusting creature with human upper body and worm tail for legs crawling back and forth.");
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
