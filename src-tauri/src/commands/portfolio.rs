use crate::models::Portfolio;
use crate::services::portfolio::PortfolioService;
use tauri::{State, Manager};
use std::sync::Mutex;
use rusqlite::Connection;
use std::path::PathBuf;

pub struct AppState {
    pub db: Mutex<Connection>,
}

#[tauri::command]
pub fn create_portfolio(
    state: State<AppState>,
    name: String,
    description: Option<String>,
) -> Result<i64, String> {
    let portfolio = Portfolio::new(name, description);
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    PortfolioService::create(&conn, &portfolio)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_portfolio(state: State<AppState>, id: i64) -> Result<Portfolio, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    PortfolioService::get_by_id(&conn, id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_portfolios(state: State<AppState>) -> Result<Vec<Portfolio>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    PortfolioService::get_all(&conn)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_portfolio(
    state: State<AppState>,
    portfolio: Portfolio,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    PortfolioService::update(&conn, &portfolio)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_portfolio(state: State<AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    PortfolioService::delete(&conn, id)
        .map_err(|e| e.to_string())
}

pub fn get_db_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");
    
    std::fs::create_dir_all(&app_dir).expect("Failed to create app data directory");
    
    app_dir.join("strategos.db")
}
