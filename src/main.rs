use rand::prelude::*;
use std::thread::sleep;
use std::{thread, time};
#[macro_use] extern crate text_io;

pub struct Player {
    head: bool,
    body: bool,
    left_arm: bool,
    right_arm: bool,
    left_leg: bool,
    right_leg: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {head : false, body : false, left_arm : false, right_arm : false, left_leg : false, right_leg : false }
    }

    //This mess of if else statements checks if the body part has already been rolled
    pub fn already_has_part(&mut self, roll: i64) -> bool {
        if roll == 1 {
            if self.head == true {
                return true;
            } else {
                self.head = true;
                return false;
            }

        } else if roll == 2 {

            if self.body {
                return true;
            } else {
                self.body = true;
                return false;
            }

        } else if roll == 3 {

            if self.left_arm {
                return true;
            } else {
                self.left_arm = true;
                return false;
            }

        } else if roll == 4 {

            if self.right_arm {
                return true;
            } else {
                self.right_arm = true;
                return false;
            }

        } else if roll == 5 {

            if self.left_leg {
                return true;
            } else {
                self.left_leg = true;
                return false;
            }

        } else {

            if self.right_leg {
                return true;
            } else {
                self.right_leg = true;
                return false;
            }

        }
    }

    //Prints out whatever parts of the body the player has rolled
    pub fn print_body(&self) {
        //Draw the head
        if self.head {
            println!("  0 ");
        }
        else {
            println!();
        }

        //This unholy mess of if else statements draws the torso
        if self.body && self.left_arm && self.right_arm{
            println!("--|--");
        }
        else if self.body && self.left_arm {
            println!("--|");
        }
        else if self.body && self.right_arm{
            println!("  |--");
        }
        else if self.left_arm && self.right_arm{
            println!("--  --");
        }
        else if self.body{
            println!("  |");
        }
        else if self.left_arm{
            println!("--   ");
        }
        else if self.right_arm{
            println!("   --")
        }
        else {
            println!();
        }

        //Draw the legs
        if self.left_leg && self.right_leg {
            println!(" / \\")
        }
        else if self.left_leg {
            println!(" /");
        }
        else if self.right_leg {
            println!("   \\");
        }
        else {
            println!();
        }
    }
    //Check if player has drawn all six body elements
    pub fn winner(&mut self) -> bool{
        self.head && self.body && self.left_arm && self.right_arm && self.left_leg && self.right_leg
    }

    //Player takes a turn. Rolling until they hit a number they have already rolled.
    pub fn take_turn(&mut self) {
        let mut rng = rand::thread_rng();
        let mut roll: i64 = rng.gen_range(1, 7);

        while !self.already_has_part(roll) && !self.winner(){
            //println!("Roll is{}", roll);
            roll = rng.gen_range(0, 6);
        }

    }
}

fn main() {
    let mut turn : i32 = 1;
    let mut human: Player = Player::new();
    let mut ai : Player = Player::new();



    println!("Welcome to the game My Buddy.");
    println!("This is a simple game in which a die is rolled.");
    println!("Each value corresponds to a different body part of the Buddy being drawn.");
    println!("1: head \n2: torso \n3: left arm \n4: right arm \n5: left leg \n6: right leg");
    println!("When the die is rolled if the body part hasn't been drawn then it is added to the drawing.");
    println!("The player continues to roll the die until they can't draw a new body part.");
    println!("First player to draw their Buddy wins");
    println!("The game will automatically play out. \nIt's you versus the AI good luck!");
     println!("Enter any key to begin game:");
     let input : String = read!();

    //Play the game
    while !human.winner() || !ai.winner(){
        sleep(time::Duration::from_millis(1000));
        if turn > 0 {
            println!("\nYour turn");
            println!("Your Buddy:");
            human.take_turn();
            human.print_body();
            turn *= -1;
            if human.winner() {
                println!("Congrats player you win!");
                return;
            }
        }
        else {
            println!("\nAi's turn");
            println!("AI's Buddy:");
            ai.take_turn();
            ai.print_body();
            turn *= -1;
            if ai.winner() {
                println!("The Ai won. Better Luck next time.");
                return;
            }
        }
    }

}