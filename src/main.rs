extern crate csv;
extern crate getopts;
#[macro_use] extern crate papers;
extern crate rustc_serialize;

use papers::util::macros::*;
use papers::datatypes::record::*;
use papers::datatypes::action::*;
use papers::datatypes::progstate::*;
use getopts::Options;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::BufReader;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;

fn get_input(message : & str, target : &mut String) {
  println!("{}", message);
  io::stdin().read_line(target).expect("Failed to read line");
}

fn new_entry() -> Record {
  let mut title   : String = String::new();
  let mut authors : String = String::new();
  let mut tags    : String = String::new();
  let mut link    : String = String::new();

  get_input("Title: "   , &mut title);
  get_input("Authors: " , &mut authors);
  get_input("Tags: "    , &mut tags);
  get_input("Link: "    , &mut link);

  return Record { title   : title.trim().to_owned()
                , authors : authors.trim().to_owned()
                , tags    : tags.trim().to_owned()
                , link    : link.trim().to_owned()
                , review  : String::new()
                };

}

fn resolve_action(matches : getopts::Matches) -> Action {
  cond!( matches.opt_present("h") => { return Action::Help; }
       , matches.opt_present("a") => { return Action::Add; }
       , orelse                   => { return Action::Empty; });
}

fn print_usage(opts : Options) {
	let brief = format!("Usage: papers [options]");
	print!("{}", opts.usage(&brief));
}

fn do_init(opts : &mut Options) -> ProgState {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();
  let mut fexists = false;

	// opts.optopt("a", "add", "Add a new entry", "NAME");
	opts.optflag("a", "add", "Add a new entry");
	opts.optflag("h", "help", "print this help menu");

  let key = "HOME";
  println!("About to do some parsing.");
  // In which I write Haskell inside of Rust.
  // At some point be sure to do efficiency analysis on this.
  mdo!( matches <- opts.parse(&args[1..]).ok()
      ; path    <- withDefaultOpt!("~".to_owned(), env::var(key).ok()) //unwrap_or could work
      ; path2   <- Some(path + "/papers.csv")
      ; and ProgState { action : resolve_action(matches)
                      , file_exists : fexists 
                      , path : path2 
                      }
      ).unwrap()
}

fn main() {
	let mut opts   = Options::new();
  let mut pstate = do_init(&mut opts);
  println!("Parsing done.");
  
  let mut new_rec : Record;

  {
    println!("Action: {:?}", pstate.action);
  }

  let mut f = match OpenOptions::new().read(true).write(true).append(true).create(true).open(&pstate.path) 
                { Ok(f)  => f 
                , Err(e) => panic!("File not found or created")
                };

  // Make a CSV reader for the file.
  let mut rdr = csv::Reader::from_file(&pstate.path).expect("No papers found."); // Might panic. 

  // Make a new internal-memory writer.
  let mut wtr = csv::Writer::from_memory();

	match pstate.action 
		{ Action::Help          => print_usage(opts)
    , Action::Add           => {new_rec = new_entry(); 
                                wtr.encode(new_rec).ok().expect("CSV Writer error"); 
                                ()}
    , Action::Search(ref s) => ()
    , Action::Empty         => print_usage(opts)
		};

  let strRes = wtr.as_string();
  println!("Encoded output: {}", strRes);

  // Easiest way to get a &str into &[u8] is to go &str->String->Vec[u8]->&[u8]
  f.write_all(&strRes.to_owned().into_bytes());

  println!("-------------------------------------");
  for record in rdr.decode() {
    let record: Record = record.unwrap();
    println!("{}\n  {}\n[{}]\n", record.title, record.authors, record.tags);
  }

}

