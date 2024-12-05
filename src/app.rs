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

use crate::{context::Context, presentation::widgets, route};

pub async fn main() {
    enable_raw_mode().unwrap();

    let (cmd_tx, cmd_rx) = mpsc::channel::<String>();
    let (ctx_tx, ctx_rx) = watch::channel::<Context>(Context::default());

    let cmd_process = tokio::spawn(cmd_thread(ctx_tx.clone(), cmd_rx));
    let input_process = tokio::spawn(input_thread(cmd_tx, ctx_tx));
    let ui_process = tokio::spawn(ui_thread(ctx_rx));

    let _ = tokio::join!(cmd_process, input_process, ui_process);

    disable_raw_mode().unwrap();
}

async fn cmd_thread(ctx_tx: watch::Sender<Context>, cmd_rx: mpsc::Receiver<String>) {
    loop {
        let value = cmd_rx.recv().unwrap();
        let mut context = ctx_tx.borrow().clone();

        let commands = route::get(context.current_route()).commands;

        commands(&value, &mut context);

        ctx_tx.send(context.clone()).unwrap();

        if context.is_exit() {
            break;
        }
    }
}

async fn input_thread(cmd_tx: mpsc::Sender<String>, ctx_tx: watch::Sender<Context>) {
    loop {
        let mut context = ctx_tx.borrow().clone();

        if event::poll(Duration::from_millis(100)).unwrap() {
            let event = event::read().unwrap();

            let keymaps = route::get(context.current_route()).keymaps;

            if let Some(command) = keymaps(&event, &mut context) {
                cmd_tx.send(command).unwrap();
            }

            ctx_tx.send(context.clone()).unwrap();
        }

        if context.is_exit() {
            break;
        }
    }
}

async fn ui_thread(ctx_rx: watch::Receiver<Context>) {
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut old_context: Option<Context> = None;
    let loading_symbols = String::from("⣾⣽⣻⢿⡿⣟⣯⣷");
    let mut loading_index: usize = 0;

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
                    build(frame, &context);
                })
                .unwrap();

            old_context = Some(context.clone());
        }

        time::sleep(Duration::from_millis(100)).await;
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
}
