use axum::Json;

pub async fn index(Json(data): Json<Vec<String>>) -> String {
    let command: Vec<&str> = data.iter().map(|s| s.as_str()).collect();
    
    if command.len() < 1 {
        return "error: command is empty".to_string();
    }

    match wei_run::command(command[0], (&command[1..]).to_vec()) {
        Ok(output) => output,
        Err(e) => format!("{}", e),
    }
}
