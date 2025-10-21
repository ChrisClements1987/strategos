use crate::models::Feature;
use crate::services::feature::FeatureService;
use crate::commands::portfolio::AppState;
use tauri::State;

#[tauri::command]
pub fn create_feature(
    state: State<AppState>,
    product_id: i64,
    name: String,
    description: Option<String>,
) -> Result<i64, String> {
    let feature = Feature::new(product_id, name, description);
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    FeatureService::create(&conn, &feature)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_feature(state: State<AppState>, id: i64) -> Result<Feature, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    FeatureService::get_by_id(&conn, id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_features_by_product(state: State<AppState>, product_id: i64) -> Result<Vec<Feature>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    FeatureService::get_by_product(&conn, product_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_feature(state: State<AppState>, feature: Feature) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    FeatureService::update(&conn, &feature)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_feature(state: State<AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    FeatureService::delete(&conn, id)
        .map_err(|e| e.to_string())
}
