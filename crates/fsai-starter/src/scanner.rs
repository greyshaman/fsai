/// Abstraction the scan is describe some process is able to performe repeated operations 
/// over some iterable data structure
trait Scan<T: Iterator> {
  fn scan<T>(&self) -> Vec<T>;
}