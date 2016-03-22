#![macro_use]

#[macro_export]
macro_rules! cond{
  ($($pred:expr => $body:block),+ orelse => $default:block) => (
    $(if $pred $body else)+
    
    $default
  )
}
