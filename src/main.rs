extern crate csv;
extern crate getopts;
extern crate papers;
extern crate rustc_serialize;

use papers::papers::datatypes::Record;
use papers::papers::datatypes::Action;
use papers::papers::datatypes::ProgState;
use getopts::Options;
use std::io;
use std::env;

fn get_input(message : & str, target : &mut String) {
  println!("{}", message);
  io::stdin().read_line(target).expect("Failed to read line");
}

fn new_entry() -> Record {
  let mut title   = String::new();
  let mut authors = String::new();
  let mut tags    = String::new();
  let mut link    = String::new();

  get_input("Title: "   , &mut title);
  get_input("Authors: " , &mut authors);
  get_input("Tags: "    , &mut tags);
  get_input("Link: "    , &mut link);

  return Record { title   : title
                , authors : authors
                , tags    : tags
                , link    : link 
                , review  : String::new()
                };

}

fn resolve_action(matches : getopts::Matches) -> Action {
  if matches.opt_present("h") {
    return Action::Help;
  } 
  if matches.opt_present("a") {
    return Action::Add;
  } 
 
  return Action::Empty;
}

fn print_usage(opts : Options) {
	let brief = format!("Usage: papers [options]");
	print!("{}", opts.usage(&brief));
}

fn do_init(opts : &mut Options) -> ProgState {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();
  let mut fexists = false;

	// opts.optopt("a", "add", "Add a new entry", "NAME);
	opts.optflag("a", "add", "Add a new entry");
	opts.optflag("h", "help", "print this help menu");

 	let matches = match opts.parse(&args[1..]) {
    Ok(m)  => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  let key = "HOME";
  let mut path = match env::var(key) 
                   { Ok(val) => {fexists = true; val}
                   , Err(e)  => "~".to_owned()
                   };
  path.push_str("/papers.csv");

  return ProgState { action : resolve_action(matches)
                   , file_exists : fexists 
                   , path : path 
                   }
}

fn main() {
	let mut opts   = Options::new();
  let mut pstate = do_init(&mut opts);
  
  let mut new_rec : Record;

	match pstate.action 
		{ Action::Help          => print_usage(opts)
    , Action::Add           => {new_rec = new_entry(); ()}
    , Action::Search(ref s) => ()
    , Action::Empty         => ()
		};

  let mut rdr = match csv::Reader::from_file(pstate.path) 
                  { Ok(val) => val
                  , Err(e)  => { println!("No papers file\n"); return }
                  };

  for record in rdr.decode() {
    let record: Record = record.unwrap();
    println!("{}\n  {}\n[{}]\n", record.title, record.authors, record.tags);
  }

}

