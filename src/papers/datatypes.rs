use rustc_serialize;

#[derive(RustcDecodable)]
pub struct Record 
  { pub title   : String
  , pub authors : String
  , pub tags    : String
  , pub link    : String
  , pub review  : String
  }

pub enum Action 
  { Help
  , Add
  , Search(String)
  , Empty
  }

pub struct ProgState 
  { pub file_exists : bool
  , pub action      : Action
  , pub path        : String
  }

