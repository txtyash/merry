use reqwest::get;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use slint::PhysicalSize;
use slint::VecModel;
use std::error::Error;
slint::include_modules!();

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Defs {
    pub list: Vec<Def>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Def {
    pub word: String,
    pub definition: String,
    pub example: String,
}

#[tokio::main]
async fn main() {
    let ui = MainWindow::new().unwrap();
    let weak = ui.as_weak();
    ui.window().set_size(PhysicalSize::new(600, 400));
    ui.run().unwrap();
    ui.on_search(move |text| {
        dbg!("called");
        let ui = weak.unwrap();
        slint::spawn_local(async move {
            let defs: Defs = find(text.into()).await.unwrap();
            let defs: Vec<Definition> = defs
                .list
                .into_iter()
                .map(|def| Definition {
                    word: def.word.into(),
                    definition: def.definition.into(),
                    example: def.example.into(),
                })
                .collect();
            let model = slint::ModelRc::new(VecModel::from(defs));
            dbg!(&model);
            ui.set_definitions(model);
        })
        .unwrap();
    });
}

async fn find(query: String) -> Result<Defs> {
    let body = get(format!(
        "https://api.urbandictionary.com/v0/define?term={query}"
    ))
    .await?
    .text()
    .await?;
    Ok(serde_json::from_str::<Defs>(&body)?)
}
