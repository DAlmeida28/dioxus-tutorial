use dioxus::prelude::*;
static CSS: Asset = asset!("/assets/main.css");

#[component]
fn DogApp() -> Element {
    rsx! {
        section { id: "title", 
        h1 { "HotDog!"}
        }
        section { id: "dogview", 
        img { src:"https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
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