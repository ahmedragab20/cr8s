use serde_json::{json, Value};
use std::error::Error;

const URL: &str = "http://127.0.0.1:8000";

async fn create_test_rustacean(client: &reqwest::Client) -> Result<Value, Box<dyn Error>> {
    let response = client
        .post(&format!("{URL}/rustaceans"))
        .json(&json!({
            "name": "Test Rustacean",
            "email": "test@test.com"
        }))
        .send()
        .await?;

    assert!(response.status().is_success());
    Ok(response.json().await?)
}

/// #### Tests the GET /rustaceans endpoint
///
/// test verifies that:
/// - The endpoint is accessible
/// - Returns a successful HTTP status code (2xx)
/// - The server is properly handling GET requests for retrieving all rustaceans
#[tokio::test]
async fn test_get_rustaceans() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let _test_rustacean = create_test_rustacean(&client).await?;

    let response = client.get(&format!("{URL}/rustaceans")).send().await?;
    assert!(response.status().is_success());

    let body: Value = response.json().await?;
    assert!(body.as_array().unwrap().len() >= 1);
    Ok(())
}

/// #### Tests getting a specific rustacean by ID
#[tokio::test]
async fn test_get_rustacean_by_id() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let test_rustacean = create_test_rustacean(&client).await?;

    let response = client
        .get(&format!("{URL}/rustaceans/{}", test_rustacean["id"]))
        .send()
        .await?;
    assert!(response.status().is_success());

    let body: Value = response.json().await?;
    assert_eq!(body["id"], test_rustacean["id"]);
    assert_eq!(body["name"], test_rustacean["name"]);
    Ok(())
}

/// #### Tests getting a non-existent rustacean
#[tokio::test]
async fn test_get_rustacean_by_id_not_found() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{URL}/rustaceans/999999"))
        .send()
        .await?;
    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
    Ok(())
}

/// #### Tests creating a new rustacean
#[tokio::test]
async fn test_create_rustacean() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{URL}/rustaceans"))
        .json(&json!({
            "name": "New Rustacean",
            "email": "new@rustacean.com"
        }))
        .send()
        .await?;

    assert!(response.status().is_success());

    let body: Value = response.json().await?;
    assert_eq!(body["name"], "New Rustacean");
    assert_eq!(body["email"], "new@rustacean.com");
    Ok(())
}

/// #### Tests updating an existing rustacean
#[tokio::test]
async fn test_update_rustacean() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let test_rustacean = create_test_rustacean(&client).await?;

    let response = client
        .put(&format!("{URL}/rustaceans/{}", test_rustacean["id"]))
        .json(&json!({
            "name": "Updated Rustacean",
            "email": "updated@rustacean.com"
        }))
        .send()
        .await?;

    assert!(response.status().is_success());

    let body: Value = response.json().await?;
    assert_eq!(body["id"], test_rustacean["id"]);
    assert_eq!(body["name"], "Updated Rustacean");
    assert_eq!(body["email"], "updated@rustacean.com");
    Ok(())
}

/// #### Tests deleting a rustacean
#[tokio::test]
async fn test_delete_rustacean() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let test_rustacean = create_test_rustacean(&client).await?;

    let response = client
        .delete(&format!("{URL}/rustaceans/{}", test_rustacean["id"]))
        .send()
        .await?;

    assert!(response.status().is_success());

    // Verify the rustacean was deleted
    let get_response = client
        .get(&format!("{URL}/rustaceans/{}", test_rustacean["id"]))
        .send()
        .await?;
    assert_eq!(get_response.status(), reqwest::StatusCode::NOT_FOUND);
    Ok(())
}
