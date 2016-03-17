pub trait Command {
  fn execute(&self);
}

#[derive(Debug)]
pub struct CliCommand {
    pub args: Vec<String>,
}

impl Command for CliCommand {
  fn execute(&self) {
    println!("Some cli command");
  }
}

pub struct Execution<C> {
  command: C
}

impl<C: Command> Execution<C> {
  pub fn execute(&self) {
    self.command.execute();
  }
}

pub fn build(command: String, args: Vec<String>) -> Option<Execution<CliCommand>>{
   print!("{}", command);
   let execution = Execution{ command: CliCommand{ args: args }};
   return Some(execution);
   // return match command.as_ref() {
   //    "add" => Some(CliCommand{
   //               args: args
   //             }),
   //       _  => None
   // }
}
