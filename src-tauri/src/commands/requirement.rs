use crate::models::Requirement;
use crate::services::requirement::RequirementService;
use crate::commands::portfolio::AppState;
use tauri::State;

#[tauri::command]
pub fn create_requirement(
    state: State<AppState>,
    client_id: i64,
    name: String,
    description: Option<String>,
) -> Result<i64, String> {
    let requirement = Requirement::new(client_id, name, description);
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    RequirementService::create(&conn, &requirement)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_requirement(state: State<AppState>, id: i64) -> Result<Requirement, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    RequirementService::get_by_id(&conn, id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_requirements_by_client(state: State<AppState>, client_id: i64) -> Result<Vec<Requirement>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    RequirementService::get_by_client(&conn, client_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_requirement(state: State<AppState>, requirement: Requirement) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    RequirementService::update(&conn, &requirement)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_requirement(state: State<AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    RequirementService::delete(&conn, id)
        .map_err(|e| e.to_string())
}
