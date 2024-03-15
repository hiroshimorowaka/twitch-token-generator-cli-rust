use inquire::Text;
use serde_json::Value;
pub async fn generate_token() {
    println!("\nGenerate Token\n");

    let client_id = Text::new("What's your Client Id?")
        .with_help_message("Insert your Client Id")
        .with_placeholder("Client id...")
        .prompt()
        .unwrap();

    let client_secret = Text::new("What's your Client Secret?")
        .with_help_message("Insert your Client Secret")
        .prompt()
        .unwrap();

    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("grant_type", "client_credentials".to_string()),
    ];
    let client = reqwest::Client::new();
    let res = client
        .post("https://id.twitch.tv/oauth2/token")
        .form(&params)
        .send()
        .await;

    if let Err(request_error) = res {
        println!("\nERROR: {request_error}\n");
        return;
    };

    let result = res.unwrap().text().await.unwrap();

    let api_response: Value = serde_json::from_str(result.as_str()).unwrap();

    let token = api_response.get("access_token");

    if token.is_none() {
        let text = format!(
            "API -> Error: {:?}",
            api_response.get("message").unwrap().as_str().unwrap()
        );
        println!("{text}");
        return;
    }
    let token_unwraped = token.unwrap();
    println!("\nYour token: {token_unwraped}\n");
    return;
}

pub async fn revoke_token() {
    println!("\nRevoke Token\n");

    let client_id = Text::new("What's your Client Id?")
        .with_help_message("Insert your Client Id")
        .with_placeholder("Client id...")
        .prompt()
        .unwrap();

    let token = Text::new("What's your Token?")
        .with_help_message("Insert your Token")
        .with_placeholder("Token...")
        .prompt()
        .unwrap();

    let params = [("client_id", client_id), ("token", token)];
    let client = reqwest::Client::new();
    let res = client
        .post("https://id.twitch.tv/oauth2/revoke")
        .form(&params)
        .send()
        .await;

    if let Err(request_error) = res {
        println!("\nERROR: {request_error}\n");
        return;
    };

    let result = res.unwrap();
    let response_status = result.status();

    if response_status != reqwest::StatusCode::OK {
        let api_response: Value =
            serde_json::from_str(result.text().await.unwrap().as_str()).unwrap();
        let message = api_response.get("message").unwrap();
        println!("\nAPI -> Error: {message}\n");
        return;
    }
    println!("\nYour token has been revoked successfully!\n");
    return;
}
