use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Food {
    id: u32,
    name: String,
    missy_comment: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Schedule {
    date: i64,
    quantity: f64,
    food: u32,
    missy_grumpiness: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct MissyFoodSchedule {
    food: Vec<Food>,
    missy_food_schedule: Vec<Schedule>,
}

fn main() -> Result<(), std::io::Error> {
    let input_path = std::env::args().nth(1).unwrap();
    let output_path = std::env::args().nth(2).unwrap();
    let mut missy_secrets = {
        let missy_secrets = std::fs::read_to_string(&input_path)?;

        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<MissyFoodSchedule>(&missy_secrets).unwrap()
    };

    // Double the quantity for each element in 'missy_food_schedule'
    for index in 0..missy_secrets.missy_food_schedule.len() {
        missy_secrets.missy_food_schedule[index].quantity *= 2.;
    }

    // Save the JSON structure into the output file
    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&missy_secrets).unwrap(),
    )?;

    Ok(())
}