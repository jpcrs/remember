#[macro_use]
extern crate clap;

use remember::{RememberOptions, Remember};

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Joao Carrasco <joaop.carrasco@gmail.com>")
        (about: "Helps you remember your cli commands")
        (@arg TAG: -t --tag +takes_value "Tag that it's going to use")
        (@subcommand insert =>
            (about: "Insert a new entry")
            (@arg command: +required "Command that is going to be inserted")
        )
        (@subcommand list =>
            (about: "List all commands in a specific tag")
        )
        (@subcommand exec =>
            (about: "Exec a specific command")
            (@arg identifier: +required "Identifier of the command that is going to be executed")
        )
    ).get_matches();
    
    let opts = RememberOptions {
        list: matches.is_present("list"),
        exec: matches.is_present("exec"),
        insert: matches.is_present("insert")
    };
    
    let remember = Remember::new(matches, &opts);
    
    remember.run();
}
