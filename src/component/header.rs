use yew::prelude::*;



#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <p class="mb-5 text-light" style="max-width: 500px;">
            {"I'm a full-stack developer, currently based in "} 
            <span style="font-family: cursive; font-size: 1.3rem;">{"Yango"}</span> 
            {". I used to work with book "}
            <span style="font-family: cursive; font-size: 1.3rem;">{"metadata"}</span>
            {", but now I work with "}
            <span style="font-family: cursive; font-size: 1.3rem;">{"code"}</span>
            {". I'm passionate about "}
            <span style="font-family: cursive; font-size: 1.3rem;">{"JavaScript"}</span>
            {", diversity, and books."}
        </p>
    }
}