use std::marker::PhantomData;

struct Opened;
struct Closed;

struct FileHandle<State> {
    name: String,
    content: Vec<String>,
    _state: PhantomData<State>,
}

fn open_file(name: &str) -> FileHandle<Opened> {
    FileHandle {
        name: name.to_string(),
        content: vec!["line1".into(), "line2".into(), "line3".into()],
        _state: PhantomData,
    }
}

impl FileHandle<Opened> {
    fn read_line(&self, n: usize) -> Option<&str> {
        self.content.get(n).map(|s| s.as_str())
    }

    fn close(self) -> FileHandle<Closed> {
        FileHandle {
            name: self.name,
            content: vec![],
            _state: PhantomData,
        }
    }
}

fn main() {
    let f = open_file("data.txt");
    println!("{}", f.read_line(0).unwrap());
    println!("{}", f.read_line(1).unwrap());
    let _closed = f.close();
    // _closed.read_line(0);  // WON'T COMPILE — type system prevents it!
    println!("File safely closed");
}

/* Output:
   line1
   line2
   File safely closed
*/
