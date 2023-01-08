/*
CERAMIC.RS by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We load Yew's APIs.
use yew::prelude::*;

// We define the heading
// component.
#[function_component]
pub fn HeadingCog () -> Html {

    // The HTML to render to the DOM.
    // The app's name.
    return html!{
        <>
         <h1>{ "CERAMIC.RS" }</h1>
        </>
    };
}