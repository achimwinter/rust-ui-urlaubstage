use chrono::Datelike;
use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};
use std::collections::HashMap;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<InitialState> = LocalizedString::new("adorsys Urlaubsrechner");


#[derive(Clone, Data, Lens)]
struct InitialState {
    current_year: String,
    employment_year: String,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = InitialState {
        current_year: "2022".to_string(),
        employment_year: chrono::offset::Local::now().year().to_string(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<InitialState> {
    // a label that will determine its text based on the current app data.
    let employment_label = Label::new(|data: &InitialState,_env: &Env| format!("Year of employment: {}", data.employment_year));
    let current_year_label = Label::new(|data: &InitialState, _env: &Env| format!("Vacation days arre calculated for: {}", data.current_year));
    // a textbox that modifies `year`.
    let textbox = TextBox::new()
        .with_placeholder("The year you got employed at adorsys")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(InitialState::employment_year);

    let vacation_days_label = Label::new(
        |data: &InitialState, _env: &Env|
            format!("Your extra vacation Days: {}",
                    calculate_vacation(
                        data.employment_year.parse::<i32>().unwrap_or(0),
                        data.current_year.parse::<i32>().unwrap_or(2022))
            ));

    let current_year_textbox = TextBox::new()
        .with_placeholder("Calculate for which year")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(InitialState::current_year);



    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(employment_label)
        .with_child(current_year_label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(current_year_textbox)
        .with_child(vacation_days_label);

    // center the two widgets in the available space
    Align::centered(layout)
}

fn calculate_vacation(year_of_employment: i32, current_year: i32) -> i32 {
    if current_year < 2022 {
        return 0;
    }

    let mut gradations = HashMap::new();
    gradations.insert(2022, 3);
    gradations.insert(2023, 5);
    gradations.insert(2024, 10);
    let mut theoretical_bonus_days: i32 = 0;

    for i in year_of_employment..current_year + 1 {
        theoretical_bonus_days += 1;
    }

    let max_possible_days = *gradations.get(&current_year).unwrap_or(&10);
    if theoretical_bonus_days > max_possible_days {
        return max_possible_days;
    }

    return theoretical_bonus_days;
}