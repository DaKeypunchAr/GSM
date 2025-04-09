slint::include_modules!();
use general_store_manager::{load_store_data_from, Dealer, Product};
use slint::{Model, ModelRc, ToSharedString, VecModel};
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

    // Search Text Change Callback
    {
        let weak_store = Rc::downgrade(&store);
        let weak_app = app.as_weak();
        app.on_search_text_changed(move |new_text, mode| {
            if mode == Mode::ComparisonProductSelection || mode == Mode::DataFeedSelection {
                let store = weak_store.upgrade().unwrap();
                let store = store.borrow();
                let best_results = store.get_best_product_results_for(&new_text).unwrap();
                let recent_results = store.get_recent_product_results().unwrap();

                let best_products: Vec<_> = best_results
                    .into_iter()
                    .map(|product| slint_generatedMainWindow::ProductData {
                        brand_name: product.brand_name.to_shared_string(),
                        pack_name: product.pack_name.to_shared_string(),
                        product_name: product.product_name.to_shared_string(),
                        item_name: product.item_name.to_shared_string(),
                        ..Default::default()
                    })
                    .collect();
                let recent_products: Vec<_> = recent_results
                    .into_iter()
                    .map(|product| slint_generatedMainWindow::ProductData {
                        brand_name: product.brand_name.into(),
                        pack_name: product.pack_name.to_shared_string(),
                        product_name: product.product_name.to_shared_string(),
                        item_name: product.item_name.to_shared_string(),
                        ..Default::default()
                    })
                    .collect();

                let input = slint_generatedMainWindow::ProductSelectionInput {
                    best_results: ModelRc::new(VecModel::from(best_products)),
                    recent_results: ModelRc::new(VecModel::from(recent_products)),
                };

                let app = weak_app.upgrade().unwrap();
                app.set_input_data(slint_generatedMainWindow::Data {
                    product_selection_input: input,
                    ..Default::default()
                });
            }
        });
    }

    // Mode Change Callback
    {
        let weak_store = Rc::downgrade(&store);
        let weak_app = app.as_weak();
        app.on_mode_changed(move |mode| match mode {
            Mode::DealerSearch => {
                let store = weak_store.upgrade().unwrap();
                let store = store.borrow();
                let dealers = store.get_dealers().unwrap();
                let modified_dealers: Vec<_> = dealers
                    .into_iter()
                    .map(|dealer| slint_generatedMainWindow::DealerData {
                        first_name: dealer.first_name.to_shared_string(),
                        middle_name: dealer.middle_name.unwrap_or_default().to_shared_string(),
                        last_name: dealer.last_name.to_shared_string(),
                        country_code: dealer.country_code.to_shared_string(),
                        phone_num: dealer.phone_num.to_shared_string(),
                        ..Default::default()
                    })
                    .collect();

                let input = slint_generatedMainWindow::DealerSearchInput {
                    results: ModelRc::new(VecModel::from(modified_dealers)),
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
                let modified_products: Vec<_> = products
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
                    results: ModelRc::new(VecModel::from(modified_products)),
                };

                let app = weak_app.upgrade().unwrap();
                app.set_input_data(slint_generatedMainWindow::Data {
                    product_search_input: input,
                    ..Default::default()
                });
            }
            _ => {}
        });
    }

    // Add Dealer Callback
    {
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
    }

    // Add Product Callback
    {
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
    }

    // Product Selection Callack
    {
        let weak_app = app.as_weak();
        let weak_store = Rc::downgrade(&store);
        app.on_product_selected(move |product_data, mode| match mode {
            Mode::ComparisonTable => {
                let store = weak_store.upgrade().unwrap();
                let store = store.borrow();

                let price_pairs = store
                    .get_latest_dealer_price_pairs_for(Product {
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
                    dealer_price_pairs: slint::ModelRc::from(Rc::new(slint::VecModel::from(
                        price_pairs,
                    ))),
                };

                let input = slint_generatedMainWindow::ComparisonTableInput {
                    product_dealers_connection: dealers_conn,
                };

                let app = weak_app.upgrade().unwrap();
                app.set_input_data(slint_generatedMainWindow::Data {
                    comparison_table_input: input,
                    ..Default::default()
                });
            }
            Mode::DataFeedProcedure => {
                let store = weak_store.upgrade().unwrap();
                let store = store.borrow();

                let price_pairs = store
                    .get_latest_dealer_price_pairs_for(Product {
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
                            country_code: price_pair.0.country_code.to_shared_string(),
                            phone_num: price_pair.0.phone_num.to_shared_string(),
                            ..Default::default()
                        },
                        price: price_pair.1 as i32,
                    })
                    .collect();

                let dealers_conn = slint_generatedMainWindow::ProductDealersConnection {
                    product: product_data,
                    dealer_price_pairs: slint::ModelRc::from(Rc::new(slint::VecModel::from(
                        price_pairs,
                    ))),
                };

                let input = slint_generatedMainWindow::DataFeedInput {
                    product_dealers_connection: dealers_conn,
                    index: 0,
                };

                let app = weak_app.upgrade().unwrap();
                app.set_input_data(slint_generatedMainWindow::Data {
                    data_feed_input: input,
                    ..Default::default()
                });
            }
            _ => unreachable!(),
        });
    }

    // Data Feed Increment Callback
    {
        let weak_app = app.as_weak();
        app.on_increment_data_feed(move || {
            let app = weak_app.upgrade().unwrap();
            {
                let input_data = app.get_input_data();
                let data_feed_input = input_data.data_feed_input;

                app.set_input_data(slint_generatedMainWindow::Data {
                    data_feed_input: slint_generatedMainWindow::DataFeedInput {
                        product_dealers_connection: data_feed_input.product_dealers_connection,
                        index: data_feed_input.index + 1,
                    },
                    ..Default::default()
                });
            }

            let input_data = app.get_input_data();
            let data_feed_input = input_data.data_feed_input;
        });
    }

    // Price Changed Callback
    {
        let weak_store = Rc::downgrade(&store);
        app.on_price_changed(move |product, dealer, price| {
            let product = Product {
                item_name: product.item_name.to_string(),
                brand_name: product.brand_name.to_string(),
                product_name: product.product_name.to_string(),
                pack_name: product.pack_name.to_string(),
            };
            let dealer = Dealer {
                first_name: dealer.first_name.to_string(),
                middle_name: if dealer.middle_name.is_empty() {
                    None
                } else {
                    Some(dealer.middle_name.to_string())
                },
                last_name: dealer.last_name.to_string(),
                country_code: dealer.country_code.to_string(),
                phone_num: dealer.phone_num.to_string(),
            };

            let store = weak_store.upgrade().unwrap();
            let store = store.borrow();
            store.update_price(product, dealer, price.parse().unwrap());
        });
    }

    app.run().unwrap();
}
