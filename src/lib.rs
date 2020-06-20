use indexmap::IndexMap;
use seed::{
    browser::service::storage::{self, Storage},
    prelude::*,
    *,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
extern crate serde;
extern crate serde_json;
mod monthly_payment;

const STORAGE_KEY: &str = "mort-calc";

// ------ ------
//     Model
// ------ ------

struct Model {
    local_storage: Storage,
    data: Data,
}

#[derive(Serialize, Deserialize)]
struct Data {
    monthly_payment_models: IndexMap<Uuid, monthly_payment::Model>,
}

impl Default for Data {
    fn default() -> Self {
        let mut first_model = IndexMap::new();
        first_model.insert(Uuid::new_v4(), monthly_payment::Model::new());
        first_model.insert(Uuid::new_v4(), monthly_payment::Model::new());
        Data {
            monthly_payment_models: first_model,
        }
    }
}

// ------ ------
//     Update
// ------ ------

#[derive(Clone)]
enum Msg {
    MonthlyPayment((Uuid, monthly_payment::Msg)),
    AddMonthlyCalc(),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::MonthlyPayment((id, msg)) => {
            if let Some(sub_mod) = model.data.monthly_payment_models.get_mut(&id) {
                if let monthly_payment::OutMsg::RemoveMe = monthly_payment::update(msg, sub_mod) {
                    model.data.monthly_payment_models.remove(&id);
                }
            }
        }
        Msg::AddMonthlyCalc() => {
            model
                .data
                .monthly_payment_models
                .insert(Uuid::new_v4(), monthly_payment::Model::new());
        }
    }
    // Save data into LocalStorage. It should be optimized in a real-world application.
    storage::store_data(&model.local_storage, STORAGE_KEY, &model.data);
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl View<Msg> {
    nodes![
        h3!["Time to Retirement Calculator"],
        model
            .data
            .monthly_payment_models
            .iter()
            .map(|(id, sub_mod)| {
                let id2 = *id;
                monthly_payment::view(&sub_mod)
                    .map_msg({ move |msg| Msg::MonthlyPayment((id2, msg)) })
            })
            .collect::<Vec<Node<Msg>>>(),
        a![
            class!("btn-floating btn-large waves-effect waves-light red"),
            i![
                class!("material-icons"),
                "add"
            ],
            simple_ev(Ev::Click, Msg::AddMonthlyCalc())
        ]
    ]
}

// ------ ------
//     Local Storage, TODO: Contribute back to seed?
// ------ ------

pub enum StorageLoadError {
    CouldNotConnect,
    DecodeError(serde_json::Error),
    NoData,
}

pub fn load_data<T>(storage: &Storage, name: &str) -> Result<T, StorageLoadError>
where
    T: serde::de::DeserializeOwned,
{
    let item = storage
        .get_item(name)
        .map_err(|_| StorageLoadError::CouldNotConnect)?;

    match item {
        None => Err(StorageLoadError::NoData),
        Some(d) => {
            let mapped = serde_json::from_str(&d);
            mapped.map_err(StorageLoadError::DecodeError)
        }
    }
}

// ------ ------
//     Harness
// ------ ------

fn after_mount(_: Url, _: &mut impl Orders<Msg>) -> AfterMount<Model> {
    let local_storage = storage::get_storage().expect("get `LocalStorage`");
    let data = load_data(&local_storage, STORAGE_KEY).unwrap_or_default();
    //storage::load_data(&local_storage, STORAGE_KEY).unwrap_or_default();

    AfterMount::new(Model {
        data,
        local_storage,
    })
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
