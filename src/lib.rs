use clap::ArgMatches;

#[derive(Debug)]
pub struct RememberOptions {
    pub list: bool,
    pub insert: bool,
    pub exec: bool
}

#[derive(Debug)]
pub struct Remember<'a> {
    pub args: ArgMatches,
    opts: &'a RememberOptions
}
https://github.com/jpcrs/remember.git
impl Remember<'_> {
    pub fn new(arg: ArgMatches, opts: &RememberOptions) -> Remember {
        Remember {
            args: arg,
            opts
        }
    }
    
    pub fn run(&self) {
        match &self.opts {
            RememberOptions {list: true, .. } => self.process_list(),
            RememberOptions {insert: true, .. } => println!("Received a insert command"),
            RememberOptions {exec: true, .. } => println!("Received a exec command"),
            _ => println!("Invalid command")
        }
    }
    
    fn process_list(&self) {
        println!("{:?}", self.args.value_of("TAG"));
    }
}