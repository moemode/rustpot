use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text).select_first("title").map(|title_element| title_element.inner_html())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} is {title}"),
            None => println!("{url} has no title")
        }
    })
}