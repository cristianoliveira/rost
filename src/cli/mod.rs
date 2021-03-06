pub mod args;
pub mod commands;

use self::args::Args;
use self::commands::*;

/// # function command
///
/// Return a command based on [args] passed as param
/// or None if any command was found.
///
pub fn command(args: Args) -> Option<Box<Execution+'static>>{
    if args.cmd_add {
        return Some(Box::new(
            AddCommand{
                ip: args.arg_ip.unwrap(),
                host: args.arg_host.unwrap(),
            })
        );
    }
    if args.cmd_del {
        return Some(Box::new(
                DeleteCommand{
                    pattern: args.arg_pattern.unwrap(),
                    })
                );
    }
    if args.cmd_list {
        return Some(Box::new(ListCommand));
    }
    None
}
