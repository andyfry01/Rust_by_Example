fn main() {
    println!("{} days", 31);

    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    println!("{number:>width$}", number=1, width=6);

    println!("{number:>0width$}", number=3, width=6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    format!("Pi is roughly {}", "3.1415");
    // {
    //     #[allow(dead_code)]
    //     #derive(Debug)
    //     struct Structure(i32);
    //     println!("This struct {:?} won't print", Structure(3));
    // }
}