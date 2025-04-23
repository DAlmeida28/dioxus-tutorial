use dioxus::prelude::*;
static CSS: Asset = asset!("/assets/main.css");
// ImageAssetOptions lets us conver the .jpg to a AVIF with optimizaitons. 
// static DOGPIC: Asset = asset!("/assets/dog1.jpg", ImageAssetOptions::new().with_avif());


#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        //Open the database fromthe persisted "hotdog.db" file
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        //Create the dogs table if it doesn't already exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs(
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
                );",
            ).unwrap();
            conn
    }
}

#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;
    Ok(())
}

#[derive(Clone)]
struct TitleState(String);

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        section { id: "title",
        h1 { "{title.0}" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        section { id: "dogview", 
            img { src: img_src.cloned().unwrap_or_default()}
        }
        section { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button { id: "save", 
                onclick: move |_| async move {
                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    _ = save_dog(current).await;
                    },
                "save"
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| TitleState("DOGS!".to_string()));
    rsx! { 
        document::Stylesheet{ href: CSS }
        Title {}
        DogView {}
     }
}