mod cache;
mod constants;
mod datasources;
mod entities;
mod enums;
mod keymaps;
mod presentation;
mod states;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = datasources::configuration::get_configuration()?;
    let mut state = states::initial::initialize();

    let authorization = match datasources::slack::authorize_local().await {
        Ok(value) => value,
        Err(_) => {
            datasources::slack::authorize(
                config.slack.client_id.clone(),
                config.slack.client_secret.clone(),
            )
            .await?
        }
    };

    state.authorization = Some(authorization);

    let token = state
        .authorization
        .clone()
        .unwrap()
        .authed_user
        .access_token;

    state.channel.channels = datasources::slack::get_conversations(token.clone())
        .await?
        .iter()
        .filter(|channel| !channel.is_im && !channel.is_mpim.unwrap_or(false))
        .map(|channel| channel.to_owned())
        .collect();

    state
        .channel
        .channels
        .sort_by(|a, b| b.updated.unwrap_or(0).cmp(&a.updated.unwrap_or(0)));

    state.channel.direct_messages = datasources::slack::get_conversations(token.clone())
        .await?
        .iter()
        .filter(|channel| channel.is_im || channel.is_mpim.unwrap_or(false))
        .map(|channel| channel.to_owned())
        .collect();
    state
        .channel
        .direct_messages
        .sort_by(|a, b| b.updated.unwrap_or(0).cmp(&a.updated.unwrap_or(0)));

    state.global.members = datasources::slack::get_users_list(token.clone()).await?;

    presentation::render(config, state).await?;

    Ok(())
}
