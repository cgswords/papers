#![macro_use]

#[macro_export]
macro_rules! cond{
  (orelse => $default:block) => ($default);
  ($pred:expr => $body:block , $($tail:tt)*) => (if $pred $body else { cond!($($tail)*) })
}

#[macro_export]
macro_rules! withDefaultOpt{
  ($default:expr, $expr:expr)  => (match $expr { Some(val) => $expr, None => Some($default)})
}

#[macro_export]
macro_rules! bindOpt{
    ($left:expr, $right:expr) => (match $left { Some(val) => $right(val) , None => None })
}

#[macro_export]
macro_rules! mdo{ 
  (and $y:expr )                       => ( bindOpt!(Some($y), |x| Some(x)));
  ($x:ident <- $y:expr ; $($tail:tt)*) => ( bindOpt!($y,       |$x|   mdo!($($tail)*)));
  ($y:block ; $($tail:tt)*)            => ( { $y; mdo!($($tail)*) });
}
  
