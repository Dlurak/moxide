use crate::helpers::runs_in_tmux;
use std::borrow::Cow;
use tmux_interface::{Error, HasSession, Tmux, TmuxCommand};

pub fn attach<'a, S: Into<Cow<'a, str>>>(name: S) -> TmuxCommand<'a> {
    if runs_in_tmux() {
        TmuxCommand::switch_client().target_session(name).into()
    } else {
        TmuxCommand::attach_session().target_session(name).into()
    }
}

pub fn session_exists<'a, S: Into<Cow<'a, str>>>(name: S) -> Result<bool, Error> {
    let has_session = HasSession::new().target_session(name);
    Tmux::with_command(has_session)
        .output()
        .map(|x| x.success())
}

#[macro_export]
macro_rules! conditional_command {
    ($condition:expr, $command:expr) => {
        if $condition {
            $command.into()
        } else {
            tmux_interface::TmuxCommand::new()
        }
    };
}

fn private_get_unused_name(name: String, used: Option<u8>) -> String {
    let new_name = match used {
        Some(counter) => format!("{}({})", name, counter),
        None => name.clone(),
    };

    if session_exists(&new_name).unwrap_or(false) {
        let next_counter = used.unwrap_or(0) + 1;
        private_get_unused_name(name, Some(next_counter))
    } else {
        new_name
    }
}

pub fn get_unused_name(name: String) -> String {
    private_get_unused_name(name, None)
}
