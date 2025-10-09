use crate::GameState;

pub fn attic(state: &mut GameState, buffer: [u8; 1]) {
    if state.move_char.x == 10 {
        if buffer == [115] {
            state.move_char.y = 2;
            println!("You climb down the lamp chain. You stop by its crown.");
        } else {
            let something: &str;
            if state.move_wormman.wormman {
                something = "A wormman is crawling around down there.";
            } else {
                something = "";
            }
            println!("There's a chain lamp going down to the floor below. {}", something);
        }
    } else if state.move_char.x <= 5 {
        state.move_char.x = 5;
        if buffer == [101] && !state.item.blade_taken {
            println!("You touch the man's shoulder, to see if he needs help.\nHe coughs. \"They... they are everywhere. Be care...ful.\"\nYou try to comfort him, but his wounds tells their own tale.\n\"Here, take this. It will protect you.\"\nHe holds out a knife, a dagger with a dark blade. \"It's magi...*\"\nHe dies. You take the blade.");
            state.item.blade_taken = true;
            state.item.inventory.push("Blade".to_string());
        } else if !state.item.blade_taken {
            println!("The man stares into nothingness. He seems to be alive.");
            state.move_char.x = 5;
        } else {
            println!("What a horrible fate...");
        }
    } else if state.move_char.x >= 17 {
        state.move_char.x = 17;
        println!("You've reached a wooden wall. You can hear wormmen on the other side, fighting among themselves.");
    } 
}
