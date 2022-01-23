mod aux;
mod opts;
mod comms;
mod state;
mod event;

use std::io;
use std::fs::File;
use std::sync::mpsc::channel;
use std::error::Error;
use std::thread;
use tui::Terminal;
use tui::backend::TermionBackend;
use clap::Parser;

use crate::opts::Opts;
use crate::state::State;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    let opts = Opts::parse();
    
    if let Some(log) = opts.log {
        use log::LevelFilter;
        use simplelog::{WriteLogger, Config};

        WriteLogger::init(LevelFilter::max(), Config::default(), File::create(log).unwrap()).unwrap()
    }
    
    let (out_tx, in_rx) = {
        let cmd = opts.cmd;
        let cmd_args = opts.cmd_args;
        let json = opts.json;
        if let Some(cmd) = cmd {
            comms::stdio::open_comms(cmd, &cmd_args, json)
        } else if let Some(nng) = opts.nng {
            comms::nng::open_comms(nng, json)
        } else {
            unimplemented!()
        }
    }?;

    let mut terminal = {
        use termion::raw::IntoRawMode;
        use termion::input::MouseTerminal;
        use termion::screen::AlternateScreen;
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        Terminal::new(backend)
    }?;

    type E = Box<dyn Error + Send + Sync>;
    type R = Result<(), E>;
    
    let (state_tx, state_rx) = channel::<State>();

    let size_hndl = thread::spawn::<_, R>({
        use std::time::Duration;
        use comms::Size;
        type E = Box<dyn Error + Send + Sync>;
        let out_tx = out_tx.clone();
        move || -> Result<(), E> {
            let sleep = Duration::from_millis(250);
            let mut terminal_size = (0, 0);
            thread::sleep(sleep); // TODO: wait for SIGWINCH
            let nts = termion::terminal_size();
            if let Ok(nts) = nts {
                if terminal_size != nts {
                    terminal_size = nts;
                    out_tx.send(Size(terminal_size))?;
                }
            }
            Ok(())
        }
    });
    
    let ev_hndl = thread::spawn::<_, R>({
        use termion::input::TermRead;
        use comms::Event;
        type E = Box<dyn Error + Send + Sync>;
        type R = Result<(), E>;
        let out_tx = out_tx.clone();
        move || -> R {
            let stdin = std::io::stdin();
            let stdin = stdin.lock();
            for e in stdin .events() {
                let e = e?;
                let e = event::Event::interp_termion_event(e);
                if let Some(e) = e {
                    out_tx.send(Event(e))?;
                }
            }
            Ok(())
        }
    });
    
    let inc_hndl = thread::spawn::<_, R>({
        use comms::{Quit, BadComm, State};
        type E = Box<dyn Error + Send + Sync>;
        move || -> Result<(), E> {
            for inc in in_rx {
                match inc {
                    Quit() => { log::info!("quit"); Ok(()) }, // TODO
                    BadComm(e) => { log::warn!("bad comm {:?}", e); Ok(()) }, // TODO
                    State(s) => state_tx.send(s),
                }?;
            }
            Ok(())
        }
    });
    
    let draw_hndl = thread::spawn::<_, R>({
        type E = Box<dyn Error + Send + Sync>;
        move || -> Result<(), E> {
            for s in state_rx {
                terminal.draw(|f| f.render_widget(s, f.size()))?;
            }
            Ok(())
        }
    });
    
   size_hndl.join().unwrap()?;
   ev_hndl.join().unwrap()?;
   inc_hndl.join().unwrap()?;
   draw_hndl.join().unwrap()?;
   Ok(())

}
