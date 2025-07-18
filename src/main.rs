fn main() {
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut game:bool = true;
    let mut ladder_up:bool = false;
    let mut ladder_down:bool = false;
    let mut lantern_picked_up:bool = false;
    let mut wormman = false;
    let mut wx1 = 10;
    let mut wy1 = 1;
    let mut wx1kill:i32 = 0;
    let mut inventory:Vec<String> = Vec::new();

    println!("\nWelcome to WormmenInvasion! \n\nYou are at the ground floor. There's a brick wall to your left. \nThere's a ladder in sight, and a dim glow beyond the ladder. \n\nMove your character with wasd. Press i for inventory. Press h for help.\n");

    while game == true {
        let last = x;
        let mut choice = String::new();
        let input = std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        if choice.contains("h") {
            println!("Wormmen Invasion is a text based 2D platformer.\nMove your character using wasd. Press i for inventory. Press h for help.\n");
        }
        if choice.contains("i") {
            println!("Inventory: {:?}", inventory);
        }
        if choice.contains("a") {
            x -= 1;
        }
        if choice.contains("d") {
            x += 1;
        }
        
        if y == 0 {
            if x == 5 {
                ladder_up = true;
            }
            if x < 1 {
                println!("There's a solid brick wall to your left.");
                x = 0;
            }
            if x > 13 {
                println!("You reach a locked door.");
                x = 14;
            }
            if x == 4 || x == 6 {
                ladder_up = false;
            }
            if x == 10 {
                if lantern_picked_up == false {
                    if choice.contains("e") {
                        inventory.push("lantern".to_string());
                        lantern_picked_up = true;
                        println!("You picked up the lantern");
                } else {
                    println!("There's a body on the ground. He or she, it's hard to tell from all the gore, holds a lantern. Press \"e\" to pick it up.");
                    }
            } else {
                println!("There's a body on the ground. It looks like it has been eaten by something.");
                    }            
            }
        } else if y == 1 {
            if x == 5 {
                ladder_down = true;
                if choice.contains("s") {
                ladder_up = true;
                }
            }
            if x == 4 || x == 6 {
                ladder_down = false;
            }
            if x == 1 {
                println!("Stone stairs leads down into darkness.");
                if choice.contains("s") {
                    if inventory.contains(&"lantern".to_string()) {
                        println!("You are in a dark cellar, but his lantern guides the path.\nSome weird sounds can be heard from the tunnel to the left.\nTo the right there is a hole in the ground and a rock wall.");
                        y = -1;
                        x = 0;
                    } else {
                        println!("It's too dark to go down there.");
                    }
                }
            }
            if x < -2 {
                println!("You've reached the  Wall");
                x = -3;
            }
            if x > 18 {
                println!("You've reached a wall");
                x = 19;
            }
        } else if y == -1 {
            if x == 4 {
                println!("The hole is deep and dark, but small. You can step over or go down.");
                if choice.contains("s") {
                    println!("You jump into the hole, and land in the sewers. The stench is unbearable.");
                    y = -2;
                }
            }
            if x > 7 {
                println!("You've reached the wall. The natural rock indicates you are under ground.");
                x = 8;
            }
            if x == -6 {
                if last == -5 {
                    println!("The sounds from the darkness are getting closer.");
                }
            }
            if x == -12 {
                if last == -11 {
                    println!("Something moves in the shadows.");
                }
            }
            if x == -15 {
                println!("Wormmen fall down from the ceiling, grab you from the darkness, crawl at you on the ground. You die in terroras slimy mouths devour you.");
                break
            }
        }
        if ladder_up == true {
            if choice.contains("w") {
                ladder_up = false;
                ladder_down = true;
                y += 1;
                println!("There's a disgusting creature with human upper body and worm tail for legs crawling back and forth.");
            } else {
                println!("There's a ladder going up.");
            }
        }
        if ladder_down == true {
            if choice.contains("s") {
                ladder_down = false;
                ladder_up = true;
                y -= 1;
                println!("There's a ladder going down.");
            }
        }
        if wx1 < -2 {
            wormman = true;
        }
        if wx1 > 13 {
            wormman = false;
        }
        if wormman {
            wx1 += 1;
            wx1kill = wx1 + 1;
        } else {
            wx1 -= 1;
            wx1kill = wx1 - 1;
        }
        if wx1 == x || wx1kill == x {
            if wy1 == y {
                println!("The wormman eats you alive as you scream in terror.");
                break
            }
        }
        if choice.contains("x") {
            break
        }
        println!("{} , {}", x, y);
        if y == 1 || (x == 5 && y == 0) {
            println!("Wormman: {}, {}", wx1, wy1);
        }
    }
}


