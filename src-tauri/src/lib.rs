mod db;
mod models;
mod services;
mod commands;

use commands::portfolio::{
    AppState, create_portfolio, get_portfolio, get_all_portfolios, 
    update_portfolio, delete_portfolio, get_db_path
};
use commands::product::{
    create_product, get_product, get_products_by_portfolio,
    get_product_modules, update_product, delete_product
};
use commands::feature::{
    create_feature, get_feature, get_features_by_product,
    update_feature, delete_feature
};
use commands::licence::{
    create_licence, get_licence, get_licences_by_portfolio,
    add_feature_to_licence, remove_feature_from_licence,
    get_licence_features, update_licence, delete_licence
};
use commands::client::{
    create_client, get_client, get_clients_by_portfolio,
    update_client, delete_client
};
use commands::requirement::{
    create_requirement, get_requirement, get_requirements_by_client,
    update_requirement, delete_requirement
};
use db::init_db;
use std::sync::Mutex;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let db_path = get_db_path(app.handle());
            let conn = init_db(&db_path).expect("Failed to initialize database");
            
            app.manage(AppState {
                db: Mutex::new(conn),
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            create_portfolio,
            get_portfolio,
            get_all_portfolios,
            update_portfolio,
            delete_portfolio,
            create_product,
            get_product,
            get_products_by_portfolio,
            get_product_modules,
            update_product,
            delete_product,
            create_feature,
            get_feature,
            get_features_by_product,
            update_feature,
            delete_feature,
            create_licence,
            get_licence,
            get_licences_by_portfolio,
            add_feature_to_licence,
            remove_feature_from_licence,
            get_licence_features,
            update_licence,
            delete_licence,
            create_client,
            get_client,
            get_clients_by_portfolio,
            update_client,
            delete_client,
            create_requirement,
            get_requirement,
            get_requirements_by_client,
            update_requirement,
            delete_requirement
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
