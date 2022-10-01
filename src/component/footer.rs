use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="d-flex flex-row justify-content-between p-5">
            <div class="d-flex flex-column">
                <ul style="list-style: none;">
                    <li><a href="https://www.facebook.com/stanley.yuuto/" class="text-muted">{"facebook"}</a></li>
                    <li><a href="https://www.instagram.com/alk.neq.me/" class="text-muted">{"instagram"}</a></li>
                    <li><a href="https://github.com/alk-neq-me" class="text-muted">{"github"}</a></li>
                    <li><a href="https://github.com/alk-neq-me/card" class="text-muted">{"source ⚙️"}</a></li>
                </ul>
            </div>
        
            <div class="bar"></div>
        </div>
    }
}