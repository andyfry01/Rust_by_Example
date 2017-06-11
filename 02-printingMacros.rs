fn main() {
    println!("{} days", 31);

    println!("{0} can specify where {1} show up, but will {1} show up where {0} want it to?", "you", "this");

    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    println!("{number:>width$}", number=1, width=6);

    println!("{number:>0width$}", number=3, width=6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    let pi = 3.141592;
    let formatted_pi = format!("{:.*}", 3, pi);
    println!("Pi written out in full is {unformatted}. But to make it easy, let's say that it is is roughly {formatted}", unformatted=pi, formatted=formatted_pi);

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct {struct:?} won't print without #[derive(Debug)]", struct=Structure(3));
}
