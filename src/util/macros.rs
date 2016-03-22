#![macro_use]

#[macro_export]
macro_rules! cond{
  (orelse => $default:block) => ($default);
  ($pred:expr => $body:block , $($tail:tt)*) => (if $pred $body else { cond!($($tail)*) })
}
