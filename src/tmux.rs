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
