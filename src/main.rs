use general_store_manager::load_store_data_from;
use slint::{SharedString, ToSharedString};
use std::env;

slint::include_modules!();

fn main() {
    let app = MainWindow::new().unwrap();

    let mut cwd = env::current_dir().unwrap();
    cwd.push("store.db");
    let path = cwd.as_path();
    let store = load_store_data_from(path).expect("Failed to make store!");

    let dealers = store
        .get_dealers()
        .expect("Failed to get dealers from store!");

    let mut dealer_rows: Vec<DealerRowData> = Vec::new();

    for dealer in dealers {
        dealer_rows.push(DealerRowData {
            name: SharedString::from(dealer.name),
            phone_num: dealer.phone_num.to_shared_string(),
            price: 100,
        });
    }

    let model = std::rc::Rc::new(slint::VecModel::from(dealer_rows));
    app.set_dealer_data(model.clone().into());

    app.run().unwrap();
}
