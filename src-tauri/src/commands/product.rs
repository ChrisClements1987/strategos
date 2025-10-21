use crate::models::Product;
use crate::services::product::ProductService;
use crate::commands::portfolio::AppState;
use tauri::State;

#[tauri::command]
pub fn create_product(
    state: State<AppState>,
    portfolio_id: i64,
    parent_product_id: Option<i64>,
    name: String,
    description: Option<String>,
    product_type: String,
) -> Result<i64, String> {
    let mut product = Product::new(portfolio_id, name, description, parent_product_id);
    product.product_type = product_type;
    
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    ProductService::create(&conn, &product)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_product(state: State<AppState>, id: i64) -> Result<Product, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    ProductService::get_by_id(&conn, id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_products_by_portfolio(state: State<AppState>, portfolio_id: i64) -> Result<Vec<Product>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    ProductService::get_by_portfolio(&conn, portfolio_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_product_modules(state: State<AppState>, parent_product_id: i64) -> Result<Vec<Product>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    ProductService::get_modules(&conn, parent_product_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_product(
    state: State<AppState>,
    product: Product,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    ProductService::update(&conn, &product)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_product(state: State<AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    ProductService::delete(&conn, id)
        .map_err(|e| e.to_string())
}
