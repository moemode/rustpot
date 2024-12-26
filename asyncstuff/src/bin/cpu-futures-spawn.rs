use trpl;

// writing async fn is equivalent to writing a function which returns a future of the return type.
async fn calc(id: i64) {
// do something one billion times
    let mut _s: i64 = 0;
    for i in 0..1_000_000_000 {
        // after every 100 million iterations, print a message
        if i % 100_000_000 == 0 {
            println!("{id} at {i}");
        }
        _s += i;
    }
    println!("{id} done");
}

fn main() {
    trpl::run(async {
        // spawn tasks
        let fut1 = trpl::spawn_task(calc(1));
        let fut2 = trpl::spawn_task(calc(2));
        let fut3 = trpl::spawn_task(calc(3));
        // wait for the tasks to complete
        let res = trpl::join!(fut1, fut2, fut3);
        println!("{:?}, {:?}, {:?}", res.0, res.1, res.2);
    });
}
