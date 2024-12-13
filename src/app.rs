use std::{sync::mpsc, time::Duration};

use crossterm::{
    event, execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use tokio::{sync::watch, time};

use crate::{
    cache::Cache,
    common::enums::request::Request,
    context::Context,
    datasources,
    entities::{configuration::Configuration, slack::conversations::Channel},
    presentation::widgets,
    route,
};

pub async fn main(config: Configuration) {
    enable_raw_mode().unwrap();

    let (cmd_tx, cmd_rx) = mpsc::channel::<String>();
    let (req_tx, req_rx) = mpsc::channel::<Request>();
    let (ctx_tx, ctx_rx) = watch::channel::<Context>(Context::default());

    let cmd_process = tokio::spawn(cmd_thread(config.clone(), req_tx, ctx_tx.clone(), cmd_rx));
    let input_process = tokio::spawn(input_thread(config.clone(), cmd_tx.clone(), ctx_tx.clone()));
    let request_process = tokio::spawn(request_thread(config.clone(), req_rx, ctx_tx, cmd_tx));
    let ui_process = tokio::spawn(ui_thread(config, ctx_rx));

    let _ = tokio::join!(cmd_process, input_process, request_process, ui_process);

    disable_raw_mode().unwrap();
}

async fn cmd_thread(
    config: Configuration,
    req_tx: mpsc::Sender<Request>,
    ctx_tx: watch::Sender<Context>,
    cmd_rx: mpsc::Receiver<String>,
) {
    loop {
        let value = cmd_rx.recv().unwrap();
        let mut context = ctx_tx.borrow().clone();

        let commands = route::get(context.current_route()).commands;

        if let Some(request) = commands(&config, &value, &mut context) {
            ctx_tx.send(context.clone()).unwrap();
            req_tx.send(request).unwrap();
        } else {
            ctx_tx.send(context.clone()).unwrap();
        }

        if context.is_exit() {
            break;
        }
    }
}

async fn input_thread(
    config: Configuration,
    cmd_tx: mpsc::Sender<String>,
    ctx_tx: watch::Sender<Context>,
) {
    loop {
        let mut context = ctx_tx.borrow().clone();

        if event::poll(Duration::from_millis(10)).unwrap() {
            let event = event::read().unwrap();

            let keymaps = route::get(context.current_route()).keymaps;

            if let Some(command) = keymaps(&config, &event, &mut context) {
                ctx_tx.send(context.clone()).unwrap();
                cmd_tx.send(command).unwrap();
            } else {
                ctx_tx.send(context.clone()).unwrap();
            }
        }

        if context.is_exit() {
            break;
        }
    }
}

async fn request_thread(
    config: Configuration,
    req_rx: mpsc::Receiver<Request>,
    ctx_tx: watch::Sender<Context>,
    cmd_tx: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        if let Ok(value) = req_rx.recv_timeout(Duration::from_millis(100)) {
            let mut command: Option<String> = None;
            let mut context = ctx_tx.borrow().clone();
            context.show_loading();
            ctx_tx.send(context.clone()).unwrap();

            match value {
                Request::Authorization(callback_command) => {
                    command = Some(callback_command);
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
                    let token = authorization.authed_user.access_token.clone();
                    let mut channels = datasources::slack::get_conversations(token.clone())
                        .await?
                        .iter()
                        .filter(|channel| !channel.is_im && !channel.is_mpim.unwrap_or(false))
                        .map(|channel| channel.to_owned())
                        .collect::<Vec<Channel>>();
                    channels.sort_by(|a, b| b.updated.unwrap_or(0).cmp(&a.updated.unwrap_or(0)));

                    let mut direct_messages = datasources::slack::get_conversations(token.clone())
                        .await?
                        .iter()
                        .filter(|channel| channel.is_im || channel.is_mpim.unwrap_or(false))
                        .map(|channel| channel.to_owned())
                        .collect::<Vec<Channel>>();
                    direct_messages
                        .sort_by(|a, b| b.updated.unwrap_or(0).cmp(&a.updated.unwrap_or(0)));

                    let members = datasources::slack::get_users_list(token).await?;

                    let mut context = ctx_tx.borrow().clone();
                    context.auth = Some(authorization);
                    context.state.channel.channels = channels;
                    context.state.channel.direct_messages = direct_messages;
                    context.state.global.members = members;

                    ctx_tx.send(context).unwrap();
                }
                Request::GetConversationHistory(channel_id) => {
                    let messages = datasources::slack::get_conversations_history(
                        context.auth.clone().unwrap().authed_user.access_token,
                        channel_id,
                    )
                    .await?;

                    let mut context = ctx_tx.borrow().clone();
                    context.state.message.messages = messages;
                    ctx_tx.send(context).unwrap();
                }
                Request::GetConversationReplies => {
                    let replies = datasources::slack::get_conversations_replies(
                        context.auth.clone().unwrap().authed_user.access_token,
                        context.state.channel.opened.clone().unwrap().id,
                        context.state.message.opened.clone().unwrap().ts,
                    )
                    .await?;

                    let mut context = ctx_tx.borrow().clone();
                    context.state.thread.messages = replies;
                    ctx_tx.send(context).unwrap();
                }
            }

            let mut context = ctx_tx.borrow().clone();
            context.hide_loading();
            ctx_tx.send(context).unwrap();

            if let Some(cmd) = command {
                cmd_tx.send(cmd).unwrap();
            }
        }

        let context = ctx_tx.borrow().clone();
        if context.is_exit() {
            break;
        }
    }

    Ok(())
}

async fn ui_thread(config: Configuration, ctx_rx: watch::Receiver<Context>) {
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut old_context: Option<Context> = None;
    let loading_symbols = String::from("⣾⣽⣻⢿⡿⣟⣯⣷");
    let mut loading_index: usize = 0;

    let mut cache: Cache<'static> = Cache::new();

    loop {
        let context = ctx_rx.borrow().clone();

        if context.loading {
            widgets::loading::build(
                &terminal.size().unwrap(),
                &loading_symbols,
                &mut loading_index,
            );
        }

        if old_context.clone().is_none_or(|value| value != context) {
            if old_context.map_or(false, |context| context.loading) && !context.loading {
                execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
                terminal.clear().unwrap();
            }

            if context.is_exit() {
                break;
            }

            let build = route::get(context.current_route()).build;

            terminal
                .draw(|frame| {
                    build(&config, frame, &context, &mut cache);
                })
                .unwrap();

            old_context = Some(context.clone());
        }

        time::sleep(Duration::from_millis(10)).await;
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
}
