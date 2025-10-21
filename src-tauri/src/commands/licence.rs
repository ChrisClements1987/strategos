use crate::models::Licence;
use crate::services::licence::LicenceService;
use crate::commands::portfolio::AppState;
use tauri::State;

#[tauri::command]
pub fn create_licence(
    state: State<AppState>,
    portfolio_id: i64,
    name: String,
    description: Option<String>,
) -> Result<i64, String> {
    let licence = Licence::new(portfolio_id, name, description);
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    LicenceService::create(&conn, &licence)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_licence(state: State<AppState>, id: i64) -> Result<Licence, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    LicenceService::get_by_id(&conn, id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_licences_by_portfolio(state: State<AppState>, portfolio_id: i64) -> Result<Vec<Licence>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    LicenceService::get_by_portfolio(&conn, portfolio_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_feature_to_licence(state: State<AppState>, licence_id: i64, feature_id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    LicenceService::add_feature(&conn, licence_id, feature_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_feature_from_licence(state: State<AppState>, licence_id: i64, feature_id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    LicenceService::remove_feature(&conn, licence_id, feature_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_licence_features(state: State<AppState>, licence_id: i64) -> Result<Vec<i64>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    LicenceService::get_features(&conn, licence_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_licence(state: State<AppState>, licence: Licence) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    LicenceService::update(&conn, &licence)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_licence(state: State<AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    LicenceService::delete(&conn, id)
        .map_err(|e| e.to_string())
}
