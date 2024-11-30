use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Url,
};
use tokio::sync::oneshot;
use warp::Filter;

use crate::entities::{
    self,
    slack::{
        authorization::Authorization, conversations::Channel, messages::Message, users::Member,
    },
};

use super::cache::{get_cache, store_cache};

pub async fn authorize_local(
) -> Result<entities::slack::authorization::Authorization, Box<dyn std::error::Error>> {
    let cache_code = String::from("oauth.v2.access");
    let result: entities::slack::authorization::Authorization = get_cache(cache_code)?;

    Ok(result)
}

pub async fn authorize(
    client_id: String,
    client_secret: String,
) -> Result<Authorization, Box<dyn std::error::Error>> {
    let base_url = "https://slack.com/oauth/v2/authorize";
    let redirect_uri = "https://localhost:7777";
    let scope: Vec<&str> = vec![];
    let user_scope: Vec<&str> = vec![
        "users:read",
        "usergroups:read",
        "channels:read",
        "channels:history",
        "groups:read",
        "groups:history",
        "mpim:read",
        "mpim:history",
        "im:read",
        "im:history",
        "chat:write",
    ];

    let mut auth_url = Url::parse(base_url)?;
    let mut params: HashMap<String, String> = HashMap::new();

    params.insert("scope".to_string(), scope.join(","));
    params.insert("user_scope".to_string(), user_scope.join(","));
    params.insert("redirect_uri".to_string(), redirect_uri.to_string());
    params.insert("client_id".to_string(), client_id.clone());

    for (key, value) in params {
        auth_url
            .query_pairs_mut()
            .append_pair(key.as_str(), value.as_str());
    }

    println!("url: {:?}", auth_url.to_string());

    std::process::Command::new("bash")
        .arg("-c")
        .arg(format!("open \"{}\"", auth_url.to_string()))
        .spawn()?;

    let (tx, rx) = oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));
    let tx_clone = Arc::clone(&tx);

    let route = warp::path::end()
        .and(warp::query::raw())
        .map(move |query: String| {
            if let Some(tx) = tx_clone.lock().unwrap().take() {
                tx.send(query).ok();
            }
            "Well done! let's close this tab and go back to your terminal"
        });

    let server = warp::serve(route).bind(([127, 0, 0, 1], 7777));
    tokio::spawn(server);

    let query_result = rx.await?;
    let query_params: Vec<(&str, &str)> = query_result
        .split('&')
        .filter_map(|param| {
            let mut split = param.split('=');
            Some((split.next()?, split.next()?))
        })
        .collect();

    if let Some((_, code)) = query_params.iter().find(|&&(key, _)| key == "code") {
        println!("Authorization code: {}", code);
        let result = exchange_access(client_id, client_secret, code.to_string()).await?;
        Ok(result)
    } else {
        panic!("Invalid authorization data, please try again...");
    }
}

async fn exchange_access(
    client_id: String,
    client_secret: String,
    code: String,
) -> Result<Authorization, Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut form_data: HashMap<&str, &str> = HashMap::new();
    form_data.insert("client_id", client_id.as_str());
    form_data.insert("client_secret", client_secret.as_str());
    form_data.insert("code", code.as_str());

    let reponse = client
        .post("https://slack.com/api/oauth.v2.access")
        .form(&form_data)
        .send()
        .await?;

    let text = reponse.text().await?;

    let cache_code = String::from("oauth.v2.access");
    store_cache(cache_code.to_string(), text.clone())?;

    let result: entities::slack::authorization::Authorization =
        serde_json::from_str(&text.as_str())?;

    Ok(result)
}

pub async fn get_conversations(token: String) -> Result<Vec<Channel>, Box<dyn std::error::Error>> {
    let cache_code = String::from("users.conversations");

    println!("Get list of channels...");
    let data = match get_cache::<Vec<Channel>>(cache_code.clone()) {
        Ok(data) => data,
        Err(_) => {
            let mut result: Vec<Channel> = Vec::new();
            let mut cursor: String = String::new();

            loop {
                let client = Client::new();
                let url = "https://slack.com/api/users.conversations";
                let mut headers = HeaderMap::new();
                let mut params = HashMap::new();

                params.insert("types", "public_channel,private_channel,mpim,im");
                params.insert("exclude_archived", "true");
                params.insert("cursor", cursor.as_str());

                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(format!("Bearer {}", token).as_str())?,
                );

                let response = client
                    .get(url)
                    .headers(headers)
                    .query(&params)
                    .send()
                    .await?;

                let body_str = response.text().await?;

                let response: entities::slack::conversations::ApiResponse =
                    serde_json::from_str(body_str.as_str())?;

                result.extend(response.channels);
                cursor = response.response_metadata.next_cursor;
                if cursor.is_empty() {
                    break;
                }
            }
            let data = serde_json::to_string(&result)?;

            store_cache(cache_code, data)?;
            result
        }
    };

    Ok(data)
}

pub async fn get_conversations_history(
    token: String,
    channel: String,
) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
    let cache_code = format!("conversations.history.{}", channel);
    let cache_data = get_cache::<Vec<Message>>(cache_code.clone());

    let mut result = cache_data.unwrap_or(Vec::new());

    let oldest = result
        .last()
        .map_or("0".to_string(), |message| message.ts.clone());

    let client = Client::new();
    let url = "https://slack.com/api/conversations.history";
    let mut headers = HeaderMap::new();
    let mut params = HashMap::new();

    params.insert("channel", channel.as_str());
    params.insert("oldest", &oldest.as_str());

    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {}", token).as_str())?,
    );

    let response = client
        .get(url)
        .headers(headers)
        .query(&params)
        .send()
        .await?;

    let body_str = response.text().await?;
    let response: entities::slack::messages::ApiResponse = serde_json::from_str(body_str.as_str())?;

    let mut response_messages = response.messages.clone();
    response_messages.sort_by(|a, b| a.ts.cmp(&b.ts));

    result.extend(response_messages);

    let data = serde_json::to_string(&result)?;

    store_cache(cache_code, data)?;

    Ok(result)
}

pub async fn get_conversations_replies(
    token: String,
    channel: String,
    ts: String,
) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
    let cache_code = format!("conversations.replies.{}.{}", channel, ts);
    let cache_data = get_cache::<Vec<Message>>(cache_code.clone());

    let mut result = cache_data.unwrap_or(Vec::new());

    let parent = result
        .first()
        .map_or("0".to_string(), |message| message.ts.clone());
    let oldest = result
        .last()
        .map_or("0".to_string(), |message| message.ts.clone());

    let client = Client::new();
    let url = "https://slack.com/api/conversations.replies";
    let mut headers = HeaderMap::new();
    let mut params = HashMap::new();

    params.insert("channel", channel.as_str());
    params.insert("ts", ts.as_str());
    params.insert("oldest", &oldest.as_str());

    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {}", token).as_str())?,
    );

    let response = client
        .get(url)
        .headers(headers)
        .query(&params)
        .send()
        .await?;

    let body_str = response.text().await?;
    let response: entities::slack::messages::ApiResponse = serde_json::from_str(body_str.as_str())?;

    let mut response_messages = response.messages.clone();
    response_messages.sort_by(|a, b| a.ts.cmp(&b.ts));

    for message in response_messages {
        if message.ts != parent {
            result.push(message);
        }
    }

    let data = serde_json::to_string(&result)?;

    store_cache(cache_code, data)?;

    Ok(result)
}

pub async fn get_users_list(token: String) -> Result<Vec<Member>, Box<dyn std::error::Error>> {
    let cache_code = String::from("users.list");

    println!("Get list of user...");
    let data = match get_cache::<Vec<Member>>(cache_code.clone()) {
        Ok(data) => data,
        Err(_) => {
            let mut result: Vec<Member> = Vec::new();
            let mut cursor: String = String::new();

            loop {
                let client = Client::new();
                let url = "https://slack.com/api/users.list";
                let mut headers = HeaderMap::new();
                let mut params = HashMap::new();

                params.insert("cursor", cursor.as_str());

                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(format!("Bearer {}", token).as_str())?,
                );

                let response = client
                    .get(url)
                    .headers(headers)
                    .query(&params)
                    .send()
                    .await?;

                let body_str = response.text().await?;

                let response: entities::slack::users::ApiResponse =
                    serde_json::from_str(body_str.as_str())?;

                result.extend(response.members);
                cursor = response.response_metadata.next_cursor;

                println!("Fetch {} users...", result.len());
                if cursor.is_empty() {
                    break;
                }
            }

            let data = serde_json::to_string(&result)?;

            store_cache(cache_code, data)?;
            result
        }
    };

    Ok(data)
}

pub async fn chat_post_message(
    token: String,
    channel: String,
    text: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://slack.com/api/chat.postMessage";
    let mut headers = HeaderMap::new();
    let mut form_data: HashMap<&str, &str> = HashMap::new();

    form_data.insert("channel", channel.as_str());
    form_data.insert("text", text.as_str());
    form_data.insert("as_user", "true");

    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {}", token).as_str())?,
    );

    client
        .post(url)
        .headers(headers)
        .form(&form_data)
        .send()
        .await?;

    Ok(())
}

pub async fn chat_post_message_reply(
    token: String,
    channel: String,
    text: String,
    ts: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://slack.com/api/chat.postMessage";
    let mut headers = HeaderMap::new();
    let mut form_data: HashMap<&str, &str> = HashMap::new();

    form_data.insert("channel", channel.as_str());
    form_data.insert("text", text.as_str());
    form_data.insert("as_user", "true");
    form_data.insert("thread_ts", ts.as_str());

    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {}", token).as_str())?,
    );

    client
        .post(url)
        .headers(headers)
        .form(&form_data)
        .send()
        .await?;

    Ok(())
}
