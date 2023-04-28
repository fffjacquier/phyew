use yew::prelude::*;

struct Book {
    id: usize,
    title: String,
    author: String,
    image_url: String,
    video_url: String,
}

#[function_component(App)]
fn app() -> Html {
    let books = vec![
        Book {
            id: 1,
            title: "Dreamland".to_string(),
            author: "Bob Lazar".to_string(),
            image_url: "".to_string(),
            video_url: "".to_string(),
        },
        Book {
            id: 2,
            title: "Seul".to_string(),
            author: "Gérard d'Aboville".to_string(),
            image_url: "https://i.ytimg.com/vi/QXNiZFGWW_E/hqdefault.jpg".to_string(),
            video_url: "https://www.youtube.com/embed/QXNiZFGWW_E?start=2112".to_string(),
        },
        Book {
            id: 3,
            title: "How to read water".to_string(),
            author: "Tristan Gooley".to_string(),
            image_url: "https://m.media-amazon.com/images/I/71fuOTJKO+L._AC_UF1000,1000_QL80_.jpg".to_string(),
            video_url: "https://www.youtube.com/watch?v=jOQmN7EM5NE".to_string(),
        },
        Book {
            id: 4,
            title: "Randonnées du vertige".to_string(),
            author: "Pascal Sombardier".to_string(),
            image_url: "".to_string(),
            video_url: "".to_string(),
        },
        Book {
            id: 5,
            title: "Par les chemins".to_string(),
            author: "Robert McFarlane".to_string(),
            image_url: "".to_string(),
            video_url: "".to_string(),
        },
        Book {
            id: 6,
            title: "The search for the genuine".to_string(),
            author: "Jim Harrison".to_string(),
            image_url: "".to_string(),
            video_url: "".to_string(),
        },
    ];

    let books = books.iter().map(|book| html! {
        <li key={book.id}>{format!("{}: {}", book.author, book.title)}</li>
    }).collect::<Html>();

    html! {
        <>
            <h1>{ "Books Explorer" }</h1>
            <div>
                <h3>{"Interesting books to read"}</h3>
                <ul>
                    {books}
                </ul>
            </div>
            <div>
                <h3>{ "Gérard d'Aboville: Seul" }</h3>
                <iframe width="560" height="315" src="https://www.youtube.com/embed/QXNiZFGWW_E?start=2112" frameborder="0" allowfullscreen=true></iframe>
                <img src="https://via.placeholder.com/640x360.png?text=Book+Placeholder" alt="book thumbnail" />
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
