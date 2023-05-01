use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Book {
    id: usize,
    title: String,
    author: String,
    image_url: String,
    video_url: String,
}

#[derive(Properties, PartialEq)]
struct BooksListProps {
    books: Vec<Book>,
    on_click: Callback<Book>
}

#[function_component(BooksList)]
fn books_list(BooksListProps { books, on_click }: &BooksListProps) -> Html {
    let on_click = on_click.clone();
    books.iter().map(|book| {
        let on_book_select = {
            let on_click = on_click.clone();
            let book = book.clone();
            Callback::from(move |_| {
                on_click.emit(book.clone())
            })
        };

        html! {
            <li key={book.id}><a onclick={on_book_select}>{format!("{}: {}", book.author, book.title)}</a></li>
        }
    }).collect()
}

#[derive(Properties, PartialEq)]
struct BookDetailsProps {
    book: Book
}

#[function_component(BookDetails)]
fn book_details(BookDetailsProps { book }: &BookDetailsProps) -> Html {
    html! {
        <div>
            <h2>{ book.title.clone() } {" par "} { book.author.clone() }</h2>
            if !book.video_url.clone().trim().is_empty() {
                <iframe width="560" height="315" src={book.video_url.clone()} frameborder="0" allowfullscreen=true></iframe>
            }
            <img src={book.image_url.clone()} alt="book thumbnail" />
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let books = vec![
        Book {
            id: 1,
            title: "Dreamland".to_string(),
            author: "Bob Lazar".to_string(),
            image_url: "https://conexaoufo.com/wp-content/uploads/conexao_UFO_featured_bob-lazar.jpg".to_string(),
            video_url: "".to_string(),
        },
        Book {
            id: 2,
            title: "Seul".to_string(),
            author: "Gérard d'Aboville".to_string(),
            image_url: "https://static.fnac-static.com/multimedia/Images/FR/NR/78/95/01/103800/1540-1/tsp20221214061545/Seul.jpg".to_string(),
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
            image_url: "https://www.artmontana.fr/wp-content/uploads/2017/02/rando-vertige-pic-de-bure.jpg".to_string(),
            video_url: "".to_string(),
        },
        Book {
            id: 5,
            title: "Par les chemins".to_string(),
            author: "Robert McFarlane".to_string(),
            image_url: "https://quotefancy.com/media/wallpaper/800x450/6593210-Robert-Macfarlane-Quote-We-are-part-mineral-beings-too-our-teeth.jpg".to_string(),
            video_url: "".to_string(),
        },
        Book {
            id: 6,
            title: "The search for the genuine".to_string(),
            author: "Jim Harrison".to_string(),
            image_url: "https://www.azquotes.com/picture-quotes/quote-the-simple-act-of-opening-a-bottle-of-wine-has-brought-more-happiness-to-the-human-race-jim-harrison-81-31-45.jpg".to_string(),
            video_url: "".to_string(),
        },
    ];

    /* Fetch data
    let books = use_state(|| vec![]);

    {
        let books = books.clone();
        use_effect_with_deps(move |_| {
            let books = books.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_books = Request::get("http://fjacquier.free.fr/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                books.set(fetched_books);
            });

        }, ());
    }*/

    let selected_book = use_state(|| None);

    let on_book_select = {
        let selected_book = selected_book.clone();
        Callback::from(move |book: Book| {
            selected_book.set(Some(book))
        })
    };

    let details = selected_book.as_ref().map(|book| html! {
        <BookDetails book={book.clone()} />
    });

    html! {
        <>
            <h1>{ "Books Explorer" }</h1>
            <div>
                <h3>{"Interesting books to read"}</h3>
                <ul>
                    <BooksList books={books} on_click={on_book_select.clone()} />
                </ul>
            </div>
            { for details }
        </>
    }
}
