//! Example: Cleanup test models created by create_model examples
//!
//! Run with: cargo run --example create_model_cleanup_async

use ollama_oxide::{DeleteRequest, OllamaApiAsync, OllamaClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Cleanup Test Models ===\n");

    let client = OllamaClient::default()?;

    let models_to_delete = ["mario-test"];

    for model_name in models_to_delete {
        println!("Deleting model: {}", model_name);
        let request = DeleteRequest::new(model_name);
        match client.delete_model(&request).await {
            Ok(()) => println!("  Deleted: {}", model_name),
            Err(e) => println!("  Not found or error: {}", e),
        }
    }

    println!("\nCleanup complete!");
    Ok(())
}
