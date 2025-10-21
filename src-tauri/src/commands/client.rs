use crate::models::Client;
use crate::services::client::ClientService;
use crate::commands::portfolio::AppState;
use tauri::State;

#[tauri::command]
pub fn create_client(
    state: State<AppState>,
    portfolio_id: i64,
    name: String,
    description: Option<String>,
) -> Result<i64, String> {
    let client = Client::new(portfolio_id, name, description);
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    ClientService::create(&conn, &client)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_client(state: State<AppState>, id: i64) -> Result<Client, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    ClientService::get_by_id(&conn, id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_clients_by_portfolio(state: State<AppState>, portfolio_id: i64) -> Result<Vec<Client>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    ClientService::get_by_portfolio(&conn, portfolio_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_client(state: State<AppState>, client: Client) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    ClientService::update(&conn, &client)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_client(state: State<AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    ClientService::delete(&conn, id)
        .map_err(|e| e.to_string())
}
