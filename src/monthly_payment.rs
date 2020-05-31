use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

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
            model.annual_savings_contribution_in_dollars, parsed_savings_apr,
            model.stash, model.current_retirement_savings_in_dollars);
    out_msg
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div![
        style![St::Border => "1px solid black",
               St::Padding => unit!(1, rem),
               St::Position => "relative"
               St::MarginBottom => unit!(1, rem),
        ],
        table![tbody![
            input_row(
                "Initial Retirement Savings:",
                input![
                    attrs![
                    At::Type => "number",
                    At::Step => "1000",
                    At::Value => model.current_retirement_savings_in_dollars
                    ],
                    input_ev(Ev::Input, |str| Msg::InitialRetirementSavingsInput(str.parse().unwrap())),
                ]
            ),
            input_row(
                "Current Annual Savings Rate:",
                input![
                    attrs![
                    At::Type => "number",
                    At::Step => "1000",
                    At::Value => model.annual_savings_contribution_in_dollars
                    ],
                    input_ev(Ev::Input, |str| Msg::CurrentSavingsInput(str.parse().unwrap())),
                ]
            ),
            //         <i class="fas fa-plus-square"></i>
            input_row(
                "Savings APR:",
                number_input(
                    Msg::DecrementApr,
                    Msg::IncrementApr,
                    |str| Msg::SavingsAprInput(str),
                    &model.savings_apr
                )
            ),
            input_row(
                "Desired Annual Retirement Income:",
                div![
                    input![
                        attrs![
                        At::Type => "number",
                        At::Step => "1000",
                        At::Value => model.desired_annual_retirement_income
                        ],
                        input_ev(Ev::Input, |str| Msg::DesiredRetirementInput(str.parse().unwrap())),
                    ]
                ]
            ),
            input_row(
                "Safe Withdrawal Rate:",
                number_input(
                    Msg::DecrementSafeWithdrawalRate,
                    Msg::IncrementSafeWithdrawalRate,
                    |str| Msg::SafeWithdrawalRateInput(str),
                    &model.safe_withdrawal_rate
                )
            ),
        ]],
        p![
            "Required retirement stash: ",
            strong![model.stash.to_string()]
        ],
        p![
            "Years to retirement: ",
            strong![model.years_to_retirement.to_string()]
        ],
        button![
            style![St::Position => "absolute", St::Top => unit!(1, rem), St::Right => unit!(1, rem)],
            simple_ev(Ev::Click, Msg::RemoveClicked),
            "-"
        ]
    ]
}

fn input_row(title: &str, content: Node<Msg>) -> Node<Msg> {
    tr![td![class!("label"), title], td![content]]
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
