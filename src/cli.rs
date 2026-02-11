use std::io::{self, Write};

pub async fn run_interactive_chat() {
    println!("🤖 Rust AI Chatbot - Interactive Mode");
    println!("Type 'exit' or 'quit' to end the conversation\n");

    loop {
        print!("You: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let message = input.trim();

                if message.is_empty() {
                    continue;
                }

                if message.eq_ignore_ascii_case("exit") || message.eq_ignore_ascii_case("quit") {
                    println!("👋 Goodbye!");
                    break;
                }

                // Send request to local server
                match send_chat_request(message).await {
                    Ok(response) => {
                        println!("Bot: {}\n", response);
                    }
                    Err(e) => {
                        eprintln!("Error: {}\n", e);
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}\n", error);
                break;
            }
        }
    }
}

async fn send_chat_request(message: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "message": message
    });

    let response = client
        .post("http://localhost:3000/chat")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to server: {}. Make sure the server is running on port 3000.", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Server error: {}", error_text));
    }

    let chat_response: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    chat_response["response"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Invalid response format".to_string())
}
