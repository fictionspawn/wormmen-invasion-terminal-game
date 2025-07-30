use crate::MoveChar;

pub fn ladder_up_down(move_char: &mut MoveChar, buffer: [u8; 1]) {
    if move_char.ladder_up == true {
        if buffer ==[119] {
            move_char.ladder_up = false;
            move_char.ladder_down = true;
            move_char.y += 1;
            println!("There's a disgusting creature with human upper body and worm tail for legs crawling back and forth.");
        } else {
            println!("There's a ladder going up.");
        }
    }
    if move_char.ladder_down == true {
        if buffer == [115] {
            move_char.ladder_down = false;
            move_char.ladder_up = true;
            move_char.y -= 1;
        } else {
            println!("There's a ladder going down.");
        }
    }
}
