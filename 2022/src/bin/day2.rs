// XYZ is you
// 6 points for a victory
// 3 if its a drawer
// 0 if it's a lose
// A X Rock     1
// B Y Paper    2
// C Z Scissors 3

fn main() {
    let games: Vec<&str> = include_str!("./day2.txt").split('\n').collect();

    let mut first_score = 0;
    let mut second_score = 0;

    let rock = 1;
    let paper = 2;
    let scissors = 3;

    let loss = 0;
    let drawer = 3;
    let win = 6;

    // we have to burrow here because we would want the values to be dereferenced when this for loop ends
    for &game in &games {
        match game {
            // Rock
            "A X" => first_score += rock + drawer, // rock rock
            "A Y" => first_score += paper + win,   // rock paper
            "A Z" => first_score += scissors + loss, // rock Scissors

            // Paper
            "B X" => first_score += rock + loss,    // paper rock
            "B Y" => first_score += paper + drawer, // paper paper
            "B Z" => first_score += scissors + win, // paper Scissors

            // Scissors
            "C X" => first_score += rock + win,   // scissors rock
            "C Y" => first_score += paper + loss, // scissors paper
            "C Z" => first_score += scissors + drawer, // scissors Scissors

            "" => continue, // ignore blanks

            _ => panic!("this shouldn't happen '{:?}'", game),
        }
    }

    // XYZ is the result of the game
    // X is loss
    // Y is drawer
    // Z is win
    for game in games {
        match game {
            // Rock
            "A X" => second_score += scissors + loss, // rock rock
            "A Y" => second_score += rock + drawer,   // rock paper
            "A Z" => second_score += paper + win,     // rock Scissors

            // Paper
            "B X" => second_score += rock + loss, // paper rock
            "B Y" => second_score += paper + drawer, // paper paper
            "B Z" => second_score += scissors + win, // paper scissors

            // Scissors
            "C X" => second_score += paper + loss, // scissors paper
            "C Y" => second_score += scissors + drawer, // scissors scissors
            "C Z" => second_score += rock + win,   // scissors rock

            "" => continue, // ignore blanks

            _ => panic!("this shouldn't happen '{:?}'", game),
        }
    }

    println!("first challenge score: {}", first_score);
    println!("second challenge score: {}", second_score)
}
