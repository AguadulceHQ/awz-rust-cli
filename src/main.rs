use cli_prompts_rs::{CliPrompt, LogType, PromptSelectOption};

use crate::macros_calculator::{
    caloric_intake, caloric_treshold, macro_split, Activity, Diet, Gender, Goal, Person,
};

pub mod macros_calculator;

fn main() {
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

    let gender_options = vec![
        PromptSelectOption::new("option1", "Male"),
        PromptSelectOption::new("option2", "Female"),
    ];

    let gender_selected = cli_prompt
        .prompt_select("What is your gender?", gender_options)
        .unwrap();

    // using as_str to get a &str to compare with instead of String
    // catch all, because as_str may not return Male or Female
    let gender = match gender_selected.label.as_str() {
        "Male" => Gender::Male,
        _ => Gender::Female,
    };

    let height: u8 = cli_prompt
        .prompt_text("What is your height?")
        .unwrap()
        .parse()
        .unwrap();

    let weight: u8 = cli_prompt
        .prompt_text("What is your weight?")
        .unwrap()
        .parse()
        .unwrap();

    let activity_options = vec![
        PromptSelectOption::new("option11", "Sedentary"),
        PromptSelectOption::new("option12", "Lightly Active"),
        PromptSelectOption::new("option13", "Moderately Active"),
        PromptSelectOption::new("option14", "Super Active"),
        PromptSelectOption::new("option15", "Very Active"),
    ];

    let activity_selected: PromptSelectOption = cli_prompt
        .prompt_select("How active are you?", activity_options)
        .unwrap();

    let activity_level = match activity_selected.label.as_str() {
        "Sedentary" => Activity::Sedentary,
        "Lightly Active" => Activity::LightlyActive,
        "Moderately Active" => Activity::ModeratelyActive,
        "Super Active" => Activity::SuperActive,
        _ => Activity::VeryActive,
    };

    let luca = Person::new(name, age, gender, height, weight, activity_level);

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

    cli_prompt
        .log(&format!("Considering a {} diet, the macros should be distributed as follows: carbs {}g, protein {}g, fat {}g", &diet, carbs, protein, fat), LogType::Info)
        .unwrap();

    cli_prompt
        .outro("Thanks for using our application. Bye 👋")
        .unwrap();
}
