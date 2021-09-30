// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};

// fn main() {
//     let stdout = stdout();
//     let message = String::from("Hello fellow Rustaceans!");
//     let width = message.chars().count();

//     let mut writer = BufWriter::new(stdout.lock());
//     say(message.as_bytes(), width, &mut writer).unwrap();
// }


// use std::net::TcpStream;
// use std::net::TcpListener;
// use std::io::Read;
// use std::io::Write;
// use std::fs;
// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         handle_connection(stream);
//     }
// }
// fn handle_connection(mut stream:TcpStream) {
//     let mut buffer = [0; 1024];

//     stream.read(&mut buffer).unwrap();

//     let get = b"GET / HTTP/1.1\r\n";

//     let (status_line, filename) = if buffer.starts_with(get) {
//         ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
//     } else {
//         ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
//     };

//     let contents = fs::read_to_string(filename).unwrap();

//     let response = format!("{}\r\nContent-Length: {}\r\n\r\n", status_line, contents);

//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }

// fn main() {
//     // In general, the `{}` will be automatically replaced with any
//     // arguments. These will be stringified.
//     println!("{} days", 31);

//     // Without a suffix, 31 becomes an i32. You can change what type 31 is
//     // by providing a suffix. The number 31i64 for example has the type i64.

//     // There are various optional patterns this works with. Positional
//     // arguments can be used.
//     println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

//     // As can named arguments.
//     println!("{subject} {verb} {object}",
//              object="the lazy dog",
//              subject="the quick brown fox",
//              verb="jumps over");

//     // Special formatting can be specified after a `:`.
//     println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

//     // You can right-align text with a specified width. This will output
//     // "     1". 5 white spaces and a "1".
//     println!("{number:>width$}", number=1, width=6);

//     // You can pad numbers with extra zeroes. This will output "000001".
//     println!("{number:0>width$}", number=1, width=6);

//     // Rust even checks to make sure the correct number of arguments are
//     // used.
//     println!("My name is {0}, {1} {0}", "Bond","mrpan");
//     // FIXME ^ Add the missing argument: "James"

//     // Create a structure named `Structure` which contains an `i32`.
//     #[derive(Debug)]
//     struct Structure(i32);

//     #[derive(Debug)]
//     struct Deep(Structure);
//     // However, custom types such as this structure require more complicated
//     // handling. This will not work.
//     println!("This struct `{:?}` won't print...", Deep(Structure(3)));
//     // FIXME ^ Comment out this line.
// }

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}