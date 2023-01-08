/*
CERAMIC.RS by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We declare
// a module
// for "modules".
mod modules;

// We load Yew's API.
use yew::prelude::*;

// We load the processing component.
use modules::DataCog::DataCog;

// We load the information section.
use modules::InfoCog::InfoCog;

// We load the footer section.
use modules::FooterCog::FooterCog;

// We load the heading section.
use modules::HeadingCog::HeadingCog;

// We declare our main
// component "App".
#[function_component]
fn App() -> Html {
    return html! {
        <>
        <HeadingCog/>
        <DataCog/>
        <br/>
        <InfoCog/>
        <br/>
        <FooterCog/>
        <br/>
        </>
    };
}

// We load "App" and render it.
fn main() {
    yew::Renderer::<App>::new().render();
}