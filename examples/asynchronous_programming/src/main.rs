use trpl::Html;
use trpl::Either;
use trpl::StreamExt;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}

/*
    When rust sees an async block of code, it compiles it into an unique
    anonymous data type that implemente Future trait. When it sees
    a function marked with async, it complies it into a non-async function
    whose body is an async block.
*/
async fn page_title(url: &str) -> Option<String> 
{
    // Future trait are lazy, so we must actively tell it to await for the result
    let response_text = trpl::get(url).await.text().await;

    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}


async fn timeout <F: Future>(future: F, max_time: Duration) -> Result<F::Output, Duration>
{
    // select takes the value of the first Future that is gonna finish
    match trpl::select(future, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

// Creating a stream starting from an iteratora
async fn strems(){
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }

}