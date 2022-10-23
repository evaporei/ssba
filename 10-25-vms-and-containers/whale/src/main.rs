use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("whale")
        .about("container runtime")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Eva Pace")
        .subcommand(
            Command::new("create")
                .about("create container")
                .arg(
                    Arg::new("bundle").required(true), // .action(ArgAction::Set)
                                                       // .num_args(1..),
                )
                .arg(Arg::new("id").required(true)),
        )
        .subcommand(Command::new("start").arg(Arg::new("id").required(true)))
        .subcommand(
            Command::new("kill")
                .arg(Arg::new("id").required(true))
                .arg(Arg::new("signal").required(true)),
        )
        .subcommand(Command::new("delete").arg(Arg::new("id").required(true)))
        .subcommand(Command::new("state").arg(Arg::new("id").required(true)))
        .get_matches();
}
