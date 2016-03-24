#![macro_use]

#[macro_export]
macro_rules! cond{
  (orelse => $default:block) => ($default);
  ($pred:expr => $body:block , $($tail:tt)*) => (if $pred $body else { cond!($($tail)*) })
}

#[macro_export]
macro_rules! withDefault{
  ($default, $expr)  => (match $expr { Ok(val) => val; Err(e) => $default})
}

#[macro_export]
macro_rules! bindMaybe{
    ($left:expr, $right:stmt) => ( match $left
                           { Ok(val) => $right(val)
                           ; Err(e)  => Err(e)
                           }
                       )
}

#[macro_export]
macro_rules! mdo{ 
  ($x:ident <- $y:expr ; $($tail:tt)*) => ( bindMaybe!($y, |$x|   mdo!($($tail)*)));
  ($y:block ; $($tail:tt)*)          => ( { $y; mdo!($($tail)*) });
  ($y:expr )                        => ( Ok($y));
}
  
