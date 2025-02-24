slint::include_modules!();
use general_store_manager::{load_store_data_from, Product};
use slint::ToSharedString;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

fn main() {
    let app = MainWindow::new().unwrap();

    let mut cwd = env::current_dir().unwrap();
    cwd.push("store.db");
    let path = cwd.as_path();

    let store = Rc::new(RefCell::new(
        load_store_data_from(path).expect("Failed to make/load store!"),
    ));

    let weak_store = Rc::downgrade(&store);
    let weak_app = app.as_weak();

    app.on_search_text_changed(move |new_text, mode| {
        if mode == Mode::ComparisonProductSelection {
            let store = weak_store.upgrade().unwrap();
            let store = store.borrow();
            let best_results = store.get_best_product_results_for(&new_text).unwrap();
            let recent_results = store.get_recent_product_results().unwrap();

            let best_products: Vec<ProductData> = best_results
                .into_iter()
                .map(|product| slint_generatedMainWindow::ProductData {
                    brand_name: product.brand_name.to_shared_string(),
                    pack_name: product.pack_name.to_shared_string(),
                    product_name: product.product_name.to_shared_string(),
                    item_name: product.item_name.to_shared_string(),
                    ..Default::default()
                })
                .collect();
            let recent_products: Vec<ProductData> = recent_results
                .into_iter()
                .map(|product| slint_generatedMainWindow::ProductData {
                    brand_name: product.brand_name.to_shared_string(),
                    pack_name: product.pack_name.to_shared_string(),
                    product_name: product.product_name.to_shared_string(),
                    item_name: product.item_name.to_shared_string(),
                    ..Default::default()
                })
                .collect();

            let input = slint_generatedMainWindow::ComparisonProductSelectionInput {
                best_results: slint::ModelRc::from(Rc::new(slint::VecModel::from(best_products))),
                recent_results: slint::ModelRc::from(Rc::new(slint::VecModel::from(
                    recent_products,
                ))),
            };

            let app = weak_app.upgrade().unwrap();
            app.set_input_data(slint_generatedMainWindow::Data {
                comparison_product_selection_input: input,
                ..Default::default()
            });
        }
    });

    let weak_store = Rc::downgrade(&store);
    let weak_app = app.as_weak();
    app.on_mode_changed(move |mode| match mode {
        Mode::DealerSearch => {
            let store = weak_store.upgrade().unwrap();
            let store = store.borrow();
            let dealers = store.get_dealers().unwrap();
            let modified_dealers: Vec<DealerData> = dealers
                .into_iter()
                .map(|dealer| slint_generatedMainWindow::DealerData {
                    first_name: dealer.first_name.to_shared_string(),
                    middle_name: dealer.middle_name.unwrap_or_default().to_shared_string(),
                    last_name: dealer.last_name.to_shared_string(),
                    phone_num: dealer.phone_num.to_shared_string(),
                    ..Default::default()
                })
                .collect();

            let input = slint_generatedMainWindow::DealerSearchInput {
                results: slint::ModelRc::from(Rc::new(slint::VecModel::from(modified_dealers))),
            };
            let app = weak_app.upgrade().unwrap();
            app.set_input_data(slint_generatedMainWindow::Data {
                dealer_search_input: input,
                ..Default::default()
            });
        }
        Mode::ProductSearch => {
            let store = weak_store.upgrade().unwrap();
            let store = store.borrow();

            let products = store.get_products().unwrap();
            let modified_products: Vec<ProductData> = products
                .into_iter()
                .map(|product| slint_generatedMainWindow::ProductData {
                    brand_name: product.brand_name.to_shared_string(),
                    pack_name: product.pack_name.to_shared_string(),
                    product_name: product.product_name.to_shared_string(),
                    item_name: product.item_name.to_shared_string(),
                    ..Default::default()
                })
                .collect();

            let input = slint_generatedMainWindow::ProductSearchInput {
                results: slint::ModelRc::from(Rc::new(slint::VecModel::from(modified_products))),
            };

            let app = weak_app.upgrade().unwrap();
            app.set_input_data(slint_generatedMainWindow::Data {
                product_search_input: input,
                ..Default::default()
            });
        }
        _ => {}
    });

    let weak_store = Rc::downgrade(&store);
    app.on_add_dealer(move |dealer_data| {
        let store = weak_store.upgrade().unwrap();
        let mut store = store.borrow_mut();
        store
            .add_dealer(
                dealer_data.first_name.as_str(),
                match dealer_data.middle_name.as_str() {
                    "" => None,
                    middle_name => Some(middle_name),
                },
                dealer_data.last_name.as_str(),
                dealer_data.country_code.as_str(),
                dealer_data.phone_num.as_str(),
            )
            .unwrap();
    });

    let weak_store = Rc::downgrade(&store);
    app.on_add_product(move |product_data| {
        let store = weak_store.upgrade().unwrap();
        let mut store = store.borrow_mut();
        store
            .add_product(
                product_data.product_name.as_str(),
                product_data.brand_name.as_str(),
                product_data.item_name.as_str(),
                product_data.pack_name.as_str(),
            )
            .unwrap();
    });

    let weak_app = app.as_weak();
    let weak_store = Rc::downgrade(&store);
    app.on_product_selected(move |product_data| {
        let store = weak_store.upgrade().unwrap();
        let store = store.borrow();

        let price_pairs = store
            .get_dealer_price_pairs_for(Product {
                product_name: product_data.product_name.to_string(),
                brand_name: product_data.brand_name.to_string(),
                item_name: product_data.item_name.to_string(),
                pack_name: product_data.pack_name.to_string(),
            })
            .unwrap();

        let price_pairs: Vec<_> = price_pairs
            .into_iter()
            .map(|price_pair| slint_generatedMainWindow::DealerPricePair {
                dealer: slint_generatedMainWindow::DealerData {
                    first_name: price_pair.0.first_name.to_shared_string(),
                    middle_name: price_pair
                        .0
                        .middle_name
                        .unwrap_or_default()
                        .to_shared_string(),
                    last_name: price_pair.0.last_name.to_shared_string(),
                    phone_num: price_pair.0.phone_num.to_shared_string(),
                    ..Default::default()
                },
                price: price_pair.1 as i32,
            })
            .collect();

        let dealers_conn = slint_generatedMainWindow::ProductDealersConnection {
            product: product_data,
            dealer_price_pairs: slint::ModelRc::from(Rc::new(slint::VecModel::from(price_pairs))),
        };

        let input = slint_generatedMainWindow::ComparisonTableInput {
            product_dealers_connection: dealers_conn,
        };

        let app = weak_app.upgrade().unwrap();
        app.set_input_data(slint_generatedMainWindow::Data {
            comparison_table_input: input,
            ..Default::default()
        });
    });

    app.run().unwrap();
}
