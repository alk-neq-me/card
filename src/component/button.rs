use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: &'static str
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    html! {
        <a class="btn btn-secondary mb-5 text-black" href="https://aunglynnxyz.vercel.app/">{props.title}</a>
    }
}