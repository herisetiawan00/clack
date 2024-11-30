mod widgets;

use std::collections::HashMap;

use crossterm::{
    event, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::CrosstermBackend,
    Terminal,
};
use widgets::{
    channel, input, messages,
    section_data::{SectionData, WidgetData},
    status_line, thread,
};

use crate::{
    cache::Cache,
    datasources::slack::{
        chat_post_message, chat_post_message_reply, get_conversations_history,
        get_conversations_replies,
    },
    entities::configuration::Configuration,
    enums::section::Section,
    keymaps,
    states::State,
};

pub async fn render(
    config: Configuration,
    mut state: State,
) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut old_widget_map: HashMap<String, Vec<WidgetData>> = HashMap::new();
    let mut old_state = state.clone();
    let mut cache = Cache::new();

    let token = state
        .authorization
        .clone()
        .unwrap()
        .authed_user
        .access_token;

    loop {
        let mut frame_state = state.clone();
        let mut frame_widget_map = old_widget_map.clone();
        let show_thread = frame_state.message.opened.is_some();

        if old_state.channel.opened != frame_state.channel.opened {
            if let Some(opened) = frame_state.channel.opened.clone() {
                frame_state.message.messages =
                    get_conversations_history(token.clone(), opened.id).await?;
            }
        }

        if show_thread && old_state.message.opened != frame_state.message.opened {
            let message = frame_state.message.opened.clone().unwrap();
            frame_state.thread.messages = get_conversations_replies(
                token.clone(),
                frame_state.channel.opened.clone().unwrap().id,
                message.ts,
            )
            .await?;
        }

        if !frame_state.input.value.is_empty()
            && frame_state.input.send
            && frame_state.channel.opened != None
        {
            match frame_state.message.opened.clone() {
                Some(message) => {
                    chat_post_message_reply(
                        token.clone(),
                        frame_state.channel.opened.clone().unwrap().id,
                        frame_state.input.value.clone(),
                        message.ts,
                    )
                    .await?
                }
                None => {
                    chat_post_message(
                        token.clone(),
                        frame_state.channel.opened.clone().unwrap().id,
                        frame_state.input.value.clone(),
                    )
                    .await?
                }
            }

            frame_state.input.value = String::new();
            frame_state.input.send = false;
        }

        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100), Constraint::Length(1)])
                .split(frame.area());

            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Max(40), Constraint::Min(10)])
                .split(chunks[0]);

            let right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(40), Constraint::Length(5)])
                .split(main_chunks[1]);

            let left_chunk = main_chunks[0];
            let input_chunk = right_chunks[1];
            let status_line_chunk = chunks[1];

            let mut sections: Vec<(Rect, SectionData)> = Vec::new();
            sections.push((left_chunk, channel::new()));
            sections.push((input_chunk, input::new()));
            sections.push((status_line_chunk, status_line::new()));

            if show_thread {
                let messages_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                    .split(right_chunks[0]);

                sections.push((messages_chunks[0], messages::new()));
                sections.push((messages_chunks[1], thread::new()));
            } else {
                let messages_chunk = right_chunks[0];
                sections.push((messages_chunk, messages::new()));
                frame_widget_map.remove(&Section::Thread.to_string());
            }

            for (chunk, section) in sections {
                if (section.need_render)(&old_state, &frame_state)
                    || !frame_widget_map.contains_key(&section.section.to_string())
                {
                    frame_widget_map.insert(
                        section.section.to_string(),
                        (section.render)(chunk, &config, &frame_state, &mut cache),
                    );
                }
            }

            for (_, widgets) in frame_widget_map.clone() {
                for WidgetData { rect, widget } in widgets {
                    widget.render(frame, rect);
                }
            }
        })?;

        let key_event = event::read()?;
        keymaps::generic(key_event.clone(), &config, &mut frame_state);
        keymaps::channels(key_event.clone(), &config, &mut frame_state);
        keymaps::messages(key_event.clone(), &config, &mut frame_state);
        keymaps::input(key_event.clone(), &config, &mut frame_state);
        keymaps::thread(key_event.clone(), &config, &mut frame_state);

        old_widget_map = frame_widget_map;
        old_state = state;
        state = frame_state;

        if state.global.exit {
            break;
        }
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
