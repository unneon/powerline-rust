extern crate powerline;

#[cfg(feature = "time")]
use powerline::modules::Time;
use powerline::modules::*;
use powerline::theme::SimpleTheme;

fn main() {
    let mut prompt = powerline::Powerline::new();

    #[cfg(feature = "time")]
    prompt.add_module(Time::<SimpleTheme>::with_time_format("%H:%M:%S"))?;

    if users::get_user_by_uid(users::get_current_uid()).unwrap().name() != "unneon" {
        prompt.add_module(User::<SimpleTheme>::new());
    }
    prompt.add_module(Host::<SimpleTheme>::show_on_remote_shell());
    prompt.add_module(Cwd::<SimpleTheme>::new(45, 4, false));
    prompt.add_module(Git::<SimpleTheme>::new());
    prompt.add_module(Cmd::<SimpleTheme>::new());
    // prompt.add_module(VirtualEnv::<SimpleTheme>::new())?;
    // prompt.add_module(ExitCode::<SimpleTheme>::new())?;

    println!("{} ", prompt);
}
