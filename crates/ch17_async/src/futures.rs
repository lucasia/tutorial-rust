use log::info;
use trpl::{Either, Html};

pub fn futures(url1: &str, url2: &str) {
    trpl::block_on(async {
        let title_fut_1 = page_title(url1);
        let title_fut_2 = page_title(url2);

        let (url, maybe_title) =
            match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };


        info!("{url} returned first");

        match maybe_title {
            Some(title) => info!("Title was '{title}'"),
            None => info!("It had not title"),
        }
    })

}

pub async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;

    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());

    (url, title)
}