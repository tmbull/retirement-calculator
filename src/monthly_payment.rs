use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};
use std::fmt::Display;


// ------ ------
//     Model
// ------ ------
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Model {
    annual_savings_contribution_in_dollars: String,
    desired_annual_retirement_income: String,
    stash: Option<f64>,
    years_to_retirement: Option<f64>,
    savings_apr: String,
    safe_withdrawal_rate: String,
    initial_retirement_savings_in_dollars: String,
}

impl Model {
    pub fn new() -> Self {
        let annual_savings_contribution_in_dollars = 10000.0;
        let desired_annual_retirement_income = 100000.0;
        let savings_apr = 5.0;
        let safe_withdrawal_rate = 4.0;
        let current_retirement_savings_in_dollars = 0.0;
        let stash = calc_required_stash(
            desired_annual_retirement_income, safe_withdrawal_rate);
        Model {
            annual_savings_contribution_in_dollars: annual_savings_contribution_in_dollars.to_string(),
            desired_annual_retirement_income: desired_annual_retirement_income.to_string(),
            years_to_retirement: Some(calc_years_to_retirement(
                annual_savings_contribution_in_dollars, savings_apr,
                stash,current_retirement_savings_in_dollars)),
            stash: Some(stash),
            savings_apr: savings_apr.to_string(),
            safe_withdrawal_rate: safe_withdrawal_rate.to_string(),
            initial_retirement_savings_in_dollars: current_retirement_savings_in_dollars.to_string(),
        }
    }
}

// ------ ------
//     Update
// ------ ------

pub enum OutMsg {
    NoOp,
    RemoveMe,
}

#[derive(Clone, Debug)]
pub enum Msg {
    InitialRetirementSavingsInput(String),
    CurrentSavingsInput(String),
    DesiredRetirementInput(String),
    SavingsAprInput(String),
    SafeWithdrawalRateInput(String),
    RemoveClicked,
}

pub fn update(msg: Msg, model: &mut Model) -> OutMsg {
    let out_msg = match msg {
        Msg::RemoveClicked => OutMsg::RemoveMe,
        _ => OutMsg::NoOp,
    };
    match msg {
        Msg::InitialRetirementSavingsInput(inp) => model.initial_retirement_savings_in_dollars = inp,
        Msg::CurrentSavingsInput(inp) => model.annual_savings_contribution_in_dollars = inp,
        Msg::DesiredRetirementInput(inp) => model.desired_annual_retirement_income = inp,
        Msg::SavingsAprInput(inp) => model.savings_apr = inp,
        Msg::SafeWithdrawalRateInput(inp) => model.safe_withdrawal_rate = inp,
        Msg::RemoveClicked => (),
    };
    let calc_result = model.desired_annual_retirement_income.parse().and_then(|retirement_income| {
        model.annual_savings_contribution_in_dollars.parse().and_then(|annual_contribution| {
            model.initial_retirement_savings_in_dollars.parse().and_then(|initial_savings| {
                model.safe_withdrawal_rate.parse().and_then(|safe_withdrawal_rate| {
                    model.savings_apr.parse().and_then(|savings_apr| {
                        let stash = calc_required_stash(retirement_income, safe_withdrawal_rate);
                        let years_to_retirement = calc_years_to_retirement(
                            annual_contribution, savings_apr,stash, initial_savings);
                        Ok((stash, years_to_retirement))
                    })
                })
            })
        })
    });
    match calc_result {
        Ok((stash, years)) => {
            model.stash = Some(stash);
            model.years_to_retirement = Some(years);
        }
        _ => {
            model.stash = None;
            model.years_to_retirement = None;
        }
    }
    out_msg
}

// ------ ------
//     View
// ------ ------

pub fn option_to_string(option: Option<impl Display>) -> String {
    match option {
        Some(num) => num.to_string(),
        None => "--".to_string()
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    let years_to_retirement_str = option_to_string(model.years_to_retirement);
    let stash_str = option_to_string(model.stash);
    div![
        class!("row"),
        div![
            class!("col s12"),
            div![
                class!("card"),
                div![
                    class!("card-content"),
                    materialize_number_input(
                        "init_savings",
                        |str| Msg::InitialRetirementSavingsInput(str),
                        "Initial Retirement Savings",
                        &model.initial_retirement_savings_in_dollars
                    ),
                    materialize_number_input(
                        "current_savings_rate",
                        |str| Msg::CurrentSavingsInput(str.parse().unwrap()),
                        "Current Annual Savings Rate",
                        &model.annual_savings_contribution_in_dollars
                    ),
                    materialize_number_input(
                        "savings_apr",
                        |str| Msg::SavingsAprInput(str.parse().unwrap()),
                        "Savings APR",
                        &model.savings_apr
                    ),
                    materialize_number_input(
                        "desired_retirement_income",
                        |str| Msg::DesiredRetirementInput(str.parse().unwrap()),
                        "Desired Annual Retirement Income",
                        &model.desired_annual_retirement_income
                    ),
                    materialize_number_input(
                        "safe_withdrawal_rate",
                        |str| Msg::SafeWithdrawalRateInput(str.parse().unwrap()),
                        "Safe Withdrawal Rate",
                        &model.safe_withdrawal_rate
                    ),
                    a![
                        class!("btn-floating halfway-fab waves-effect waves-light red"),
                        i![
                            class!("material-icons"),
                            "remove"
                        ],
                        simple_ev(Ev::Click, Msg::RemoveClicked),
                    ],
                ],
                div![
                    class!("card-action grey lighten-4"),
                    p![
                        "Required retirement stash: ",
                        strong![stash_str]
                    ],
                    p![
                        "Years to retirement: ",
                        strong![years_to_retirement_str]
                    ]
                ]
            ]
        ]
    ]
}

fn materialize_number_input(
    id: &str, update_msg: fn(String) -> Msg, label: &str, text: &str) -> Node<Msg> {
    let validation = if text.is_empty() {
        ""
    } else {
        match text.parse::<f64>() {
            Ok(_) => "valid",
            Err(_) => "invalid",
        }
    };
    div![
        class!("row"),
        div![
            class!("input-field"),
            // "$",
            input![
                class!["validate", validation],
                attrs![
                    At::Type => "text",
                    At::Value => text,
                    At::Id => id,
                    "inputmode" => "number"
                ],
                input_ev(Ev::Input, move |str| update_msg(str)),
            ],
            label![
                class!("active"),
                attrs![
                    At::For => id
                ],
                label
            ],
            span![
                class!("helper-text"),
                attrs![
                    "data-error" => "Input is not a number"
                ]
            ]
        ]
    ]
}

fn calc_years_to_retirement(savings_contribution_per_year: f64, savings_apr: f64,
                            desired_stash: f64,
                            initial_investment: f64) -> f64 {
    let i = savings_apr / 100.0; // TODO: Will need to change this for monthly contribution
    let big_a = savings_contribution_per_year; // Deposit amount
    let big_x = desired_stash; // Target savings
    let big_b = initial_investment; // Initial balance
    let years = (1.0 / (1.0 + i).ln()) * ((big_a + big_x * i).ln() - (big_a + big_b * i).ln());

    round_to_cents(years)
}

fn calc_required_stash(desired_retirement_income: f64, safe_withdrawal_rate: f64) -> f64 {
    let stash = desired_retirement_income / (safe_withdrawal_rate / 100.0);
    round_to_cents(stash)
}

fn round_to_cents(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}
