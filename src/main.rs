mod youtube;

fn main() {
    //println!("Hello, world!");

    let results = youtube::search("mental outlaw", 2);

    for res in results.unwrap() {
        println!("ID: {}, Title: {}", res.id, res.title);
    }
}
