use examplestdlibrary;
use examplestdlibrary::colors::*;
fn main() {
    let result = examplestdlibrary::read_stdin();
    println!("{}", result);
    println!("{}", green("This text will be printed in green"));
}
