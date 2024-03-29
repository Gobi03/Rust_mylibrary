// https://doc.rust-lang.org/std/collections/struct.BTreeSet.html

let mut books = BTreeSet::new();

books.insert("Odyssey");
books.remove("Odyssey");

books.contains("Odyssey"); // => true

// to Vec
books.into_iter().collect::<Vec<_>>();

// from
let set = BTreeSet::from([1, 2, 3]);
