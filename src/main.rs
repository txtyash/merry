use mewe::{search, Search};
use slint::{PhysicalSize, SharedString};

slint::include_modules!();
#[tokio::main]
async fn main() {
    let ui = MerryWindow::new().unwrap();
    let weak = ui.as_weak();
    ui.on_search(move |text| {
        dbg!("clicked!");
        let ui = weak.unwrap();
        let query = text.to_string();
        dbg!(&query);
        slint::spawn_local(async move {
            dbg!(&query);
            let response = match search(query).await {
                Ok(Search::Definition(d)) => dbg!(d),
                Ok(Search::Suggestions(s)) => dbg!(s),
                Err(e) => dbg!(e.to_string()),
            };
            dbg!(&response);
            ui.set_response(SharedString::from(response));
        })
        .unwrap();
    });
    ui.window().set_size(PhysicalSize::new(400, 500));
    ui.run().unwrap();
}
