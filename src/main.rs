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

fn main() {
    println!("Hello battleship!");

let chances = 3;
if chances == 0 {
    println!("You have no chances left");
    return;
}

let mut rng = rand::thread_rng();
let random_num = rng.gen_range(1..=3);

let mut spot1 = vec![ ];
let mut spot2 = vec![ ];
let mut spot3 = vec![ ];


let mut spots = vec!["A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "B5", "C1", "C2", "C3", "C4", "C5", "D1", "D2", "D3", "D4", "D5", "E1", "E2", "E3", "E4", "E5"];




if random_num == 1 {
spot1.push("A1");
spot1.push("A2");

spot2.push ("D1");
spot2.push ("E1");

spot3.push ("D3");
spots.push ("D4");
}

else if random_num == 2 {
    spot1.push("A3");
    spot1.push("B3");
    
    spot2.push ("C3");
    spot2.push ("D3");
    
    spot3.push ("E3");
    spot3.push ("E2");
}

else if random_num == 3 {
    spot1.push("A2");
    spot1.push("B2");
    
    spot2.push ("B4");
    spot2.push ("B5");
    
    spot3.push ("E4");
    spot3.push ("E5");
}

println!("Pick a spot. try to sink all the ship!")

println!("Available spots: {:?}", spots);




}
