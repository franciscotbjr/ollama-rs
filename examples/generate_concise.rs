//! Example: Concise responses using stop sequences
//!
//! Demonstrates how to use StopSetting to prevent model rambling
//! in a banking assistant scenario.
//!
//! Run with: cargo run --example generate_concise
//!
//! Based on: impl/10-post-generate-case.md

use ollama_oxide::{GenerateRequest, ModelOptions, OllamaApiAsync, OllamaClient, StopSetting};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllamaClient::default()?;
    let model = "qwen3:0.6b";

    // System prompt for concise banking assistant
    let system_prompt = r#"Strict instructions:
1. Answer ONLY with the requested information
2. Do not make comparisons with other banks
3. Do not offer additional information or products
4. Do not explain unsolicited concepts
5. Maximum format: 1-2 sentences"#;

    // Stop sequences to prevent rambling
    let stop_sequences = StopSetting::multiple([
        "\n\n",            // Prevents new paragraphs
        " Additionally",   // Prevents "Additionally..."
        " Comparing",      // Prevents comparisons
        " It's worth",     // Prevents unnecessary emphasis
        " It's important", // Prevents additional explanations
        " I can",          // Prevents help offers
        " By the way",     // Prevents cross-selling
    ]);

    // Model options for controlled generation
    let options = ModelOptions::new()
        .with_temperature(0.3) // Low temperature for consistency
        .with_num_predict(100) // Limit output tokens
        .with_stop(stop_sequences);

    println!("=== Banking Assistant (Concise Responses) ===\n");

    // Example 1: Interest rate question
    println!("Question: What is the overdraft rate?");
    let request = GenerateRequest::new(model, "What is your bank's overdraft interest rate?")
        .with_system(system_prompt)
        .with_options(options.clone());

    let response = client.generate(&request).await?;
    println!("Response: {}\n", response.text().unwrap_or("No response"));

    // Example 2: Account balance question
    println!("Question: How do I check my balance?");
    let request = GenerateRequest::new(model, "How do I check my account balance?")
        .with_system(system_prompt)
        .with_options(options.clone());

    let response = client.generate(&request).await?;
    println!("Response: {}\n", response.text().unwrap_or("No response"));

    // Example 3: What is inflation (educational, still concise)
    println!("Question: What is inflation?");
    let request = GenerateRequest::new(model, "What is inflation?")
        .with_system(system_prompt)
        .with_options(options);

    let response = client.generate(&request).await?;
    println!("Response: {}\n", response.text().unwrap_or("No response"));

    println!("=== Demo Complete ===");
    println!("The 'stop' parameter helps cut off rambling early,");
    println!("keeping responses focused and objective.");

    Ok(())
}
