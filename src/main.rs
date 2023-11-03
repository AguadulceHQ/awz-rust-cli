use cli_prompts_rs::{CliPrompt, LogType, PromptSelectOption};
use std::process::exit;

use crate::macros_calculator::{
    caloric_intake, caloric_treshold, macro_split, Activity, Diet, Gender, Goal, Person,
};

pub mod macros_calculator;

fn main() {
    let luca = Person::new(
        String::from("Luca"),
        33,
        Gender::Male,
        173,
        70,
        Activity::SuperActive,
    );

    println!("{}", luca);

    let caloric_treshold = caloric_treshold(luca);

    println!("Luca's caloric treshold is {}", caloric_treshold);

    let goal = Goal::WeightLoss;

    println!(
        "Luca's caloric intake should be {} for {}",
        caloric_intake(caloric_treshold, &goal),
        goal
    );

    let diet = Diet::LowCarb;
    let (carbs, protein, fat) =
        macro_split(caloric_intake(caloric_treshold, &goal), &diet).to_grams();

    println!("Considering a {} diet, the macros should be distributed as follows: carbs {}g, protein {}g, fat {}g", &diet, carbs, protein, fat);

    let mut cli_prompt = CliPrompt::new();
    cli_prompt.intro("This is a basic CLI tool").unwrap();

    let name = cli_prompt.prompt_text("What is your name?").unwrap();

    cli_prompt
        .log(&format!("Your name is {}", &name), LogType::Info)
        .unwrap();

    let age: u8 = cli_prompt
        .prompt_text("What is your age?")
        .unwrap()
        .parse()
        .unwrap();

    cli_prompt
        .log(&format!("You are {} years old", &age), LogType::Info)
        .unwrap();

    cli_prompt
        .log(&format!("Adulthood: {}", is_adult(age)), LogType::Info)
        .unwrap();

    let answer = cli_prompt
        .prompt_confirm("Is the information filled correct?")
        .unwrap();

    if !answer {
        cli_prompt.cancel("Operation cancelled").unwrap();
        exit(0);
    }

    let options = vec![
        PromptSelectOption::new("option1", "Sedentary"),
        PromptSelectOption::new("option2", "Lightly Active"),
        PromptSelectOption::new("option3", "Moderately Active"),
        PromptSelectOption::new("option4", "Super Active"),
        PromptSelectOption::new("option5", "Very Active"),
    ];

    let selected_option = cli_prompt
        .prompt_select("How active are you?", options)
        .unwrap();

    cli_prompt
        .log(&format!("{}", selected_option), LogType::Info)
        .unwrap();
    cli_prompt
        .outro("Thanks for using our application. Bye 👋")
        .unwrap();
}

fn is_adult(age: u8) -> bool {
    const THRESHOLD_FOR_MAJORITY: u8 = 18;

    if age >= THRESHOLD_FOR_MAJORITY {
        return true;
    }
    return false;
}
