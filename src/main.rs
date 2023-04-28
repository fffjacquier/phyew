use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {

    html! {
        <>
            <h1>{ "Books Explorer" }</h1>
            <div>
                <h3>{"Interesting books to read"}</h3>
                <ul>
                    <li>{ "Bob Lazar: Dreamland" }</li>
                    <li>{ "Hélène Renard: L'après-vie" }</li>
                    <li>{ "Tristan Gooley: How to read water" }</li>
                    <li>{ "Pascal Sombardier: Randonnées du vertige" }</li>
                    <li>{ "Robert McFarlane: Par les chemins" }</li>
                    <li>{ "Gérard d'Aboville: Seul" }</li>
                    <li>{ "Jim Harrison: The search for the genuine" }</li>
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
