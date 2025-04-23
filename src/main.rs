use dioxus::prelude::*;
static CSS: Asset = asset!("/assets/main.css");
static DogPic: Asset = asset!("/assets/dog1.jpg");

#[component]
fn DogApp() -> Element {
    rsx! {
        section { id: "title", 
        h1 { "HotDog!"}
        }
        section { id: "dogview", 
        img { src: DogPic }
        }
        section { id: "buttons",
            button { id: "skip", "skip"}
            button { id: "save", "save!"}
        }
    }
}
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { 
        document::Stylesheet{ href: CSS }
        DogApp {}
     }
}