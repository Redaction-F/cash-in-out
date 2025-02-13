// Vec<bool>型の値のtrueの数を数える
#[tauri::command]
pub fn count_true(vec: Vec<bool>) -> usize {
    vec.into_iter().filter(|&v| v).count()
}