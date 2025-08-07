use crate::{MoveChar, Item, WindowWormman};

pub fn first_floor(move_char: &mut MoveChar, item: &mut Item, buffer: [u8; 1]) {
    if move_char.x == 5 {
        move_char.ladder_down = true;
        if buffer == [115] {
            move_char.ladder_up = true;
        }
    }
    if move_char.x == 4 || move_char.x == 6 {
        move_char.ladder_down = false;
    }
    if move_char.x == 1 {
        println!("Stone stairs leads down into darkness.");
        if buffer == [115] {
            if item.inventory.contains(&"Lantern".to_string()) {
                println!("You are in a dark cellar, but your lantern guides the path.\nSome weird sounds can be heard from the tunnel to the left.\nTo the right there is a hole in the ground and a rock wall.");
                move_char.y = -1;
                move_char.x = 0;
            } else {
                println!("It's too dark to go down there.");
            }
        }
    }
    if move_char.x < -2 {
        println!("You've reached the  Wall");
        move_char.x = -3;
    }
    if move_char.x > 13 {
        println!("You've reached a wall");
        move_char.x = 14;
    }
    if move_char.x == 10 {
         if buffer == [119] {
            println!("You step on the table,jump up and hang on to the lamp.\nThe wormman crawls around on the floor below. He seems to have forgotten all about you.");
            move_char.y = 2; 
       } else {
        println!("There's a table section with some chairs.\nA lamp is hanging in a chain from the ceiling.");
        }
    }
}

pub fn climb_lamp(move_char: &mut MoveChar, buffer: [u8;1]) {
    if buffer == [115] {
        println!("You drop yourself back down on the stone floor.");
        move_char.y = 1; 
                
    }
    if buffer ==[119] {
    println!("You are in some kind of attic. Some crates and rubbish lie around.\nA man sits against the wall by a window by the wall to your left.");
    move_char.y = 3;
    }
    if move_char.x < 10 || move_char.x > 10 {
                move_char.x = 10;
    }
}

pub fn attic(move_char: &mut MoveChar, window_wormman: &mut WindowWormman, item: &mut Item, buffer: [u8; 1], death: &mut bool) {
    if move_char.x == 10 {
        if buffer == [115] {
            move_char.y = 2;
        } else {
        println!("There's a chain lamp going down to the floor below. A wormman is crawling around down there.");
        }
    } else if move_char.x <= 5 {
        if buffer == [101] && !item.blade_taken {
            println!("You touch the man's shoulder, to see if he needs help.\nHe coughs. \"They... they are everywhere. Be care...ful.\"\nYou try to confort him, but his wounds tells their own tale.\n\"Here, take this. It will protect you.\"\nHe holds out a knife, a dagger with a dark blade. \"It's magi...*\"\nHe dies. You take the blade.");
            item.blade_taken = true;
        } else if !item.blade_taken {
            println!("The man stares into nothingness. He seems to be alive.");
            move_char.x = 5;
        } else {
            println!("What a horrible fate...");
    }
    }
    if move_char.x == 7 && item.blade_taken && !window_wormman.wormman {
        println!("A wormman falls in through the window and comes crawling towards you.");
        window_wormman.wormman = true;
        window_wormman.x = 12;
        window_wormman.y = 3;
        }
    if window_wormman.wormman {
        window_wormman.x -= 1;
        window_wormman.wwkill = window_wormman.x - 1;
        println!("Wormman: {}, {}", window_wormman.x, window_wormman.y);
        if window_wormman.x == move_char.x || window_wormman.wwkill == move_char.x {
            *death = true;
            println!("The wormmman devours you. You die in horror.");
        }
    }
}
