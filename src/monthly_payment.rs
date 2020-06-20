use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};
use std::fmt::Display;


// ------ ------
//     Model
// ------ ------
#[derive(Default, Serialize, Deserialize)]
pub struct Model {
    annual_savings_contribution_in_dollars: f64,
    desired_annual_retirement_income: f64,
    stash: f64,
    years_to_retirement: f64,
    savings_apr: String,
    safe_withdrawal_rate: String,
    current_retirement_savings_in_dollars: f64,
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
            annual_savings_contribution_in_dollars,
            desired_annual_retirement_income,
            years_to_retirement: calc_years_to_retirement(
                annual_savings_contribution_in_dollars, savings_apr,
                stash,current_retirement_savings_in_dollars),
            stash,
            savings_apr: savings_apr.to_string(),
            safe_withdrawal_rate: safe_withdrawal_rate.to_string(),
            current_retirement_savings_in_dollars,
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

#[derive(Clone)]
pub enum Msg {
    InitialRetirementSavingsInput(f64),
    CurrentSavingsInput(f64),
    DesiredRetirementInput(f64),
    DecrementApr,
    SavingsAprInput(String),
    IncrementApr,
    DecrementSafeWithdrawalRate,
    SafeWithdrawalRateInput(String),
    IncrementSafeWithdrawalRate,
    RemoveClicked,
}

pub fn update(msg: Msg, model: &mut Model) -> OutMsg {
    let out_msg = match msg {
        Msg::RemoveClicked => OutMsg::RemoveMe,
        _ => OutMsg::NoOp,
    };
    let parsed_savings_apr = model.savings_apr.parse::<f64>().unwrap();
    let parsed_safe_withdrawal_rate = model.safe_withdrawal_rate.parse::<f64>().unwrap();
    match msg {
        Msg::InitialRetirementSavingsInput(inp) => model.current_retirement_savings_in_dollars = inp,
        Msg::CurrentSavingsInput(inp) => model.annual_savings_contribution_in_dollars = inp,
        Msg::DesiredRetirementInput(inp) => model.desired_annual_retirement_income = inp,
        // TODO: better error handling
        Msg::DecrementApr => {
            model.savings_apr = (parsed_savings_apr - 1.0).to_string();
            log!("{}", model.savings_apr)
        },
        Msg::SavingsAprInput(inp) => model.savings_apr = inp,
        Msg::IncrementApr => model.savings_apr = (parsed_savings_apr + 1.0).to_string(),
        Msg::SafeWithdrawalRateInput(inp) => model.safe_withdrawal_rate = inp,
        Msg::RemoveClicked => (),
        Msg::DecrementSafeWithdrawalRate => model.safe_withdrawal_rate = (parsed_safe_withdrawal_rate - 1.0).to_string(),
        Msg::IncrementSafeWithdrawalRate => model.safe_withdrawal_rate = (parsed_safe_withdrawal_rate + 1.0).to_string(),
    };
    model.stash = calc_required_stash(
        model.desired_annual_retirement_income, model.safe_withdrawal_rate.parse().unwrap());
    model.years_to_retirement =
        calc_years_to_retirement(
            model.annual_savings_contribution_in_dollars, model.savings_apr.parse().unwrap(),
            model.stash, model.current_retirement_savings_in_dollars);
    out_msg
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div![
        class!("row"),
        div![
            class!("col s12"),
            div![
                class!("card"),
                div![
                    class!("card-content"),
                    materialize_text_input(
                        "init_savings",
                        |str| Msg::InitialRetirementSavingsInput(str.parse().unwrap()),
                        "Initial Retirement Savings",
                        &model.current_retirement_savings_in_dollars
                    ),
                    materialize_text_input(
                        "current_savings_rate",
                        |str| Msg::CurrentSavingsInput(str.parse().unwrap()),
                        "Current Annual Savings Rate",
                        &model.annual_savings_contribution_in_dollars
                    ),
                    materialize_text_input(
                        "savings_apr",
                        |str| Msg::SavingsAprInput(str.parse().unwrap()),
                        "Savings APR",
                        &model.savings_apr
                    ),
                    // input_row(
                    //     "Savings APR:",
                    //     number_input(
                    //         Msg::DecrementApr,
                    //         Msg::IncrementApr,
                    //         |str| Msg::SavingsAprInput(str),
                    //         &model.savings_apr
                    //     )
                    // ),
                    materialize_text_input(
                        "desired_retirement_income",
                        |str| Msg::DesiredRetirementInput(str.parse().unwrap()),
                        "Desired Annual Retirement Income",
                        &model.desired_annual_retirement_income
                    ),
                    materialize_text_input(
                        "safe_withdrawal_rate",
                        |str| Msg::SafeWithdrawalRateInput(str.parse().unwrap()),
                        "Safe Withdrawal Rate",
                        &model.safe_withdrawal_rate
                    ),
                    // input_row(
                    //     "Safe Withdrawal Rate:",
                    //     number_input(
                    //         Msg::DecrementSafeWithdrawalRate,
                    //         Msg::IncrementSafeWithdrawalRate,
                    //         |str| Msg::SafeWithdrawalRateInput(str),
                    //         &model.safe_withdrawal_rate
                    //     )
                    // ),
                    a![
                        class!("btn-floating halfway-fab waves-effect waves-light red"),
                        i![
                            class!("material-icons"),
                            "remove"
                        ],
                        simple_ev(Ev::Click, Msg::RemoveClicked),
                    ],
                ],
                //           <a class="btn-floating halfway-fab waves-effect waves-light red"><i class="material-icons">add</i></a>
                div![
                    class!("card-action grey lighten-4"),
                    p![
                        "Required retirement stash: ",
                        strong![model.stash.to_string()]
                    ],
                    p![
                        "Years to retirement: ",
                        strong![model.years_to_retirement.to_string()]
                    ]
                ]
            ]
        ]
    ]
}

fn input_row(title: &str, content: Node<Msg>) -> Node<Msg> {
    tr![td![class!("label"), title], td![content]]
}

fn materialize_text_input(
    id: &str, update_msg: fn(String) -> Msg, label: &str, text: &impl Display) -> Node<Msg> {
    div![
        class!("row"),
        div![
            class!("input-field"),
            // "$",
            input![
                class!("validate"),
                attrs![
                    At::Type => "number",
                    At::Value => text,
                    At::Id => id
                ],
                input_ev(Ev::Input, move |str| update_msg(str))
            ],
            label![
                class!("active"),
                attrs![
                    At::For => id
                ],
                label
            ]
        ]
    ]
}

fn number_input(decrement_msg: Msg, increment_msg: Msg, update_msg: fn(String) -> Msg, display_value: &str) -> Node<Msg> {
    div![
        class!("calc-input"),
        i![
            class!["fas", "fa-minus-square"],
            simple_ev("click", decrement_msg)
        ],
        input![
            attrs![
            At::Type => "number",
            At::Step => "1",
            At::Value => display_value
            ],
            input_ev(Ev::Input, move |str| update_msg(str)),
        ],
        div![class!("suffix"), "%"],
        i![
            class!["fas", "fa-plus-square"],
            simple_ev("click", increment_msg)
        ],
    ]
}

// ------ ------
//     Calc
// ------ ------

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
