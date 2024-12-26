struct Excerpt<'a, 'b: 'a> {
    part: &'a str,
    whole: &'b str,
}

impl<'a, 'b: 'a> Excerpt<'a, 'b> {
    fn new(part: &'a str, whole: &'b str) -> Self {
        Excerpt { part, whole }
    }

    fn part_of_whole(&self) -> &'a str {
        self.part
    }
}

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let first_sentence = text.split('.').next().expect("Could not find a '.'");
    let excerpt = Excerpt::new(first_sentence, &text);

    println!("Excerpt: {}", excerpt.part_of_whole());
}