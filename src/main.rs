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
    let random_num: i32 = rng.random_range(1..=3);
    println!("Dice roll: {}", random_num);

    println!("Hello battleship!");

    let chances = 3;
    if chances == 0 {
        println!("You have no chances left");
        return;
    }

    let mut ship1 = vec![];
    let mut ship2 = vec![];
    let mut ship3 = vec![];

    let mut spots = vec![
        "A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "B5", "C1", "C2", "C3", "C4", "C5", "D1", "D2", "D3", "D4", "D5", "E1", "E2", "E3",
        "E4", "E5",
    ];

    if random_num == 1 {
        ship1.push("A1");
        ship1.push("A2");

        ship2.push("D1");
        ship2.push("E1");

        ship3.push("D3");
        spots.push("D4");
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
    // println!("Available spots: {:?}", spots);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    println!("You entered: {}", input);

    if input == "a1" || input == "A1" && ship1.contains(&"A1") || ship2.contains(&"A1") || ship3.contains(&"A1") {
        println!("You hit the ship!");
    } else {
        println!("You hit! the water..");
    }
}
