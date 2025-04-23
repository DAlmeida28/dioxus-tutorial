use dioxus::prelude::*;
static CSS: Asset = asset!("/assets/main.css");
// ImageAssetOptions lets us conver the .jpg to a AVIF with optimizaitons. 
// static DOGPIC: Asset = asset!("/assets/dog1.jpg", ImageAssetOptions::new().with_avif());

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
            button { onclick: move |_| img_src.restart(), id: "save", "save" }
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