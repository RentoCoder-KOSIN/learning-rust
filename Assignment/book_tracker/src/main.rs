#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
    finished: bool,
}

fn new_book(title: String, pages: u32) -> Book {
    Book {
        title,
        pages,
        finished: false,
    }
}

impl Book {
    fn summary(&self) -> String {
        format!(
            "title:{}, pages:{} finished:{}",
            self.title, self.pages, self.finished
        )
    }

    fn finish(&mut self) {
        self.finished = true;
    }
}

fn main() {
    let mut book1 = new_book(String::from("Started struct!"), 40);

    println!("{}", book1.summary());
    book1.finish();
    println!("{}", book1.summary());
    println!("{:?}", book1);
}
