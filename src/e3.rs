use rand::Rng;

enum Situation {
    Sang(u8),
    Kaghaz(u8),
    Gheychi(u8),
}
//helllo

pub fn mainfn() {
    let mut rn = rand::thread_rng();
    let mut rng = rand::thread_rng();
    let avali = rn.gen_range(1..4);
    let dovomi = rng.gen_range(1..4);

    let player1 = match avali {
        1 => Situation::Sang(avali),
        2 => Situation::Kaghaz(avali),
        3 => Situation::Gheychi(avali),
        _ => unreachable!(),
    };

    let player2 = match dovomi {
        1 => Situation::Sang(dovomi),
        2 => Situation::Kaghaz(dovomi),
        3 => Situation::Gheychi(dovomi),
        _ => unreachable!(),
    };

    println!("Avali: {}", get_name(&player1));
    println!("Dovomi: {}", get_name(&player2));

    if let Situation::Sang(_) = player1 {
        if let Situation::Gheychi(_) = player2 {
            println!("Player 1 wins!");
        } else if let Situation::Kaghaz(_) = player2 {
            println!("Player 2 wins!");
        } else {
            println!("It's a tie!");
        }
    } else if let Situation::Kaghaz(_) = player1 {
        if let Situation::Sang(_) = player2 {
            println!("Player 1 wins!");
        } else if let Situation::Gheychi(_) = player2 {
            println!("Player 2 wins!");
        } else {
            println!("It's a tie!");
        }
    } else if let Situation::Gheychi(_) = player1 {
        if let Situation::Kaghaz(_) = player2 {
            println!("Player 1 wins!");
        } else if let Situation::Sang(_) = player2 {
            println!("Player 2 wins!");
        } else {
            println!("It's a tie!");
        }
    }
}

fn get_name(s: &Situation) -> &str {
    match s {
        Situation::Sang(_) => "Sang",
        Situation::Kaghaz(_) => "Kaghaz",
        Situation::Gheychi(_) => "Gheychi",
    }
}
