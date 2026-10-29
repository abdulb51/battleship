/*
By: <Abdul Baig>
Date: 2026-09-15
Program Details: <This is a simple 1 player version of battleship.

grid of a minimum of 25 boxs
Label each grid by writing numbers across the top and letters down the side, so that the squares are easily identified as A8 or F5.
The computer should randomly places two or three “ships” in the grid.
Each ship should be at least 2 boxs in size
The player only gets so many misses to get it right.
It must make sure you cannot guess the same box twice
There must be comments in the code
must say how many turns it took to win / lose
opening, winning and losing messages>
*/

use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::rng();
    let random_num: i32 = rng.random_range(1..=1);
    println!("Dice roll: {}", random_num);

    println!("Hello battleship!");
    let mut spots = vec![
        "A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "B5", "C1", "C2", "C3", "C4", "C5", "D1", "D2", "D3", "D4", "D5", "E1", "E2", "E3",
        "E4", "E5",
    ];
    let mut chances = 10;

    let mut ship1 = vec![];
    let mut ship2 = vec![];
    let mut ship3 = vec![];





    if random_num == 1 {
        ship1.push("A1");
        ship1.push("A2");

        ship2.push("D1");
        ship2.push("E1");

        ship3.push("D3");
        ship3.push("D4");
    } else if random_num == 2 {
        ship1.push("A3");
        ship1.push("B3");

        ship2.push("C3");
        ship2.push("D3");

        ship3.push("E3");
        ship3.push("E2");
    } else if random_num == 3 {
        ship1.push("A2");
        ship1.push("B2");

        ship2.push("B4");
        ship2.push("B5");

        ship3.push("E4");
        ship3.push("E5");
    }

    println!("Pick a spot. try to sink all the ship!");

    loop {
        println!("Available spots: {:?}", spots);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        if guess.trim() == "1" {
            // troble shooting to see the ships
            println!("Ship 1: {:?}", ship1);
            println!("Ship 2: {:?}", ship2);
            println!("Ship 3: {:?}", ship3);
        }






        println!("You entered: {}", guess.trim()); // just so you know what you typed but yeah

        if guess.trim() == "a1" || guess.trim() == "A1" && ship1.contains(&"A1") || ship2.contains(&"A1") || ship3.contains(&"A1") {
            println!("You hit the ship!");
            spots.remove(spots.iter().position(|&r| r == "A1").unwrap());
            ship1.remove(ship1.iter().position(|&r| r == "A1").unwrap());
        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
                spots.remove(spots.iter().position(|&r| r == "A1").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }


  if guess.trim() == "a2" || guess.trim() == "A2" && ship1.contains(&"A2") || ship2.contains(&"A2") || ship3.contains(&"A2") {
            println!("You hit the ship!");
            spots.remove(spots.iter().position(|&r| r == "A2").unwrap());
            ship1.remove(ship1.iter().position(|&r| r == "A2").unwrap());
        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
 spots.remove(spots.iter().position(|&r| r == "A2").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }


  if guess.trim() == "a3" || guess.trim() == "A3" && ship1.contains(&"A3") || ship2.contains(&"A3") || ship3.contains(&"A3") {
            println!("You hit the ship!");
            spots.remove(spots.iter().position(|&r| r == "A3").unwrap());
            ship1.remove(ship1.iter().position(|&r| r == "A3").unwrap());
        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
            spots.remove(spots.iter().position(|&r| r == "A3").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }

  if guess.trim() == "a4" || guess.trim() == "A4" && ship1.contains(&"A4") || ship2.contains(&"A4") || ship3.contains(&"A4") {
            println!("You hit the ship!");
            spots.remove(spots.iter().position(|&r| r == "A4").unwrap());
            ship1.remove(ship1.iter().position(|&r| r == "A4").unwrap());
        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
spots.remove(spots.iter().position(|&r| r == "A4").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }

          if guess.trim() == "a5" || guess.trim() == "A5" && ship1.contains(&"A5") || ship2.contains(&"A5") || ship3.contains(&"A5") {
            println!("You hit the ship!");
            spots.remove(spots.iter().position(|&r| r == "A5").unwrap());
            ship1.remove(ship1.iter().position(|&r| r == "A5").unwrap());
        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
             spots.remove(spots.iter().position(|&r| r == "A5").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }

  if guess.trim() == "b1" || guess.trim() == "B1" && ship1.contains(&"B1") || ship2.contains(&"B1") || ship3.contains(&"B1") {
            println!("You hit the ship!");
            spots.remove(spots.iter().position(|&r| r == "B1").unwrap());
            ship1.remove(ship1.iter().position(|&r| r == "B1").unwrap());
        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
 spots.remove(spots.iter().position(|&r| r == "B1").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }

        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
            spots.remove(spots.iter().position(|&r| r == "C2").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }




  if guess.trim() == "c3" || guess.trim() == "C3" && ship1.contains(&"C3") || ship2.contains(&"C3") || ship3.contains(&"C3") {
            println!("You hit the ship!");
            spots.remove(spots.iter().position(|&r| r == "C3").unwrap());
            ship1.remove(ship1.iter().position(|&r| r == "C3").unwrap());
        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
            spots.remove(spots.iter().position(|&r| r == "C3").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }


  if guess.trim() == "c4" || guess.trim() == "C4" && ship1.contains(&"C4") || ship2.contains(&"C4") || ship3.contains(&"C4") {
            println!("You hit the ship!");
            spots.remove(spots.iter().position(|&r| r == "C4").unwrap());
            ship1.remove(ship1.iter().position(|&r| r == "C4").unwrap());
        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
            spots.remove(spots.iter().position(|&r| r == "C4").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }



  if guess.trim() == "c5" || guess.trim() == "C5" && ship1.contains(&"C5") || ship2.contains(&"C5") || ship3.contains(&"C5") {
            println!("You hit the ship!");
            spots.remove(spots.iter().position(|&r| r == "C5").unwrap());
            ship1.remove(ship1.iter().position(|&r| r == "C5").unwrap());
        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
            spots.remove(spots.iter().position(|&r| r == "C5").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }


  if guess.trim() == "d1" || guess.trim() == "D1" && ship1.contains(&"D1") || ship2.contains(&"D1") || ship3.contains(&"D1") {
            println!("You hit the ship!");
            spots.remove(spots.iter().position(|&r| r == "D1").unwrap());
            ship1.remove(ship1.iter().position(|&r| r == "D1").unwrap());
        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
            spots.remove(spots.iter().position(|&r| r == "D1").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }

        if guess.trim() == "d2" || guess.trim() == "D2" && ship1.contains(&"D2") || ship2.contains(&"D2") || ship3.contains(&"D2") {
            println!("You hit the ship!");
            spots.remove(spots.iter().position(|&r| r == "D2").unwrap());
            ship1.remove(ship1.iter().position(|&r| r == "D2").unwrap());
        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
            spots.remove(spots.iter().position(|&r| r == "D2").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }
        



        if guess.trim() == "d3" || guess.trim() == "D3" && ship1.contains(&"D3") || ship2.contains(&"D3") || ship3.contains(&"D3") {
            println!("You hit the ship!");
            spots.remove(spots.iter().position(|&r| r == "D3").unwrap());
            ship1.remove(ship1.iter().position(|&r| r == "D3").unwrap());
        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
            spots.remove(spots.iter().position(|&r| r == "D3").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }

if guess.trim() == "d4" || guess.trim() == "D4" && ship1.contains(&"D4") || ship2.contains(&"D4") || ship3.contains(&"D4") {
            println!("You hit the ship!");
            spots.remove(spots.iter().position(|&r| r == "D4").unwrap());
            ship1.remove(ship1.iter().position(|&r| r == "D4").unwrap());
        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
            spots.remove(spots.iter().position(|&r| r == "D4").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }

if guess.trim() == "d5" || guess.trim() == "D5" && ship1.contains(&"D5") || ship2.contains(&"D5") || ship3.contains(&"D5") {
            println!("You hit the ship!");
            spots.remove(spots.iter().position(|&r| r == "D5").unwrap());
            ship1.remove(ship1.iter().position(|&r| r == "D5").unwrap());
        } else {
            println!(
                "You hit! the water..
        better luck next time"
            );
            spots.remove(spots.iter().position(|&r| r == "D5").unwrap());
            chances -= 1;
            println!("You have {} chances left", chances);
            println!("remaining spaces: {:?}", spots);
        }
     if chances == 0 {
            println!(
                "You have no chances left"
            );
            break;
        } 
    }




}