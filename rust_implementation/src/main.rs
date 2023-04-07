// Rust Code to play rock, paper, scissors
use rand::seq::SliceRandom;

#[derive(Debug)]
pub enum RPS {
    Rock,
    Paper,
    Scissors
}

// Logic to test for winner of each round
fn outcome(a: &RPS, b: &RPS) -> &'static str {
    match(a, b) {
        (RPS::Rock,RPS::Paper) | (RPS::Paper, RPS::Rock) => {"Paper covers rock!"}, 
        (RPS::Rock, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => {"Rock breaks scissors!"},
        (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Paper) => {"Scissors cuts paper!"},
        _ => {"Same object type!"} 
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let game_objects = [
        RPS::Rock,
        RPS::Paper,
        RPS::Scissors,
    ];
    let (mut paper_counter, mut rock_counter, mut scissors_counter, mut same_object_counter) = (0, 0, 0, 0);

    //Play for x rounds
    for _i in 0..1000000{
        let first_value = game_objects.choose(&mut rng).unwrap();
        let second_value = game_objects.choose(&mut rng).unwrap();
        let result = outcome(first_value, second_value);
        // println!("Result value: {:?}", result);
        if result == "Paper covers rock!" {
            paper_counter += 1;
        }
        else if result == "Rock breaks scissors!" {
            rock_counter += 1;
        }
        else if result == "Scissors cuts paper!" {
            scissors_counter += 1;
        }
        else if result == "Same object type!" {
            same_object_counter += 1;
        }
    }
    //Show the count across all the iterations
    println!("Rock counter value: {},\
              Paper counter value: {},\
              Scissors counter value: {},\
              Same object counter value: {}\
             ", rock_counter, paper_counter,
             scissors_counter, same_object_counter);
}
