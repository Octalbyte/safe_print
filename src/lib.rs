use std::io;
use std::io::Write;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub fn safe_print(a1: &str) {
    print!("{}", a1);
    io::stdout().flush().unwrap();
}



pub fn safe_eprint(a1: &str) {
    eprint!("{}", a1);
    io::stderr().flush().unwrap();
}