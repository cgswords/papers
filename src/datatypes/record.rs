use rustc_serialize;

#[derive(RustcEncodable,RustcDecodable)]
pub struct Record 
  { pub title   : String
  , pub authors : String
  , pub tags    : String
  , pub link    : String
  , pub review  : String
  }

