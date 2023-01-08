/*
CERAMIC.RS by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We load Yew's APIs.
use yew::prelude::*;

// We define the footer
// component.
#[function_component]
pub fn FooterCog () -> Html {

    // The HTML to render to the DOM.
    // Me being goofy and a link to the source.
    return html!{
        <>
         <p class="footer">
          { "Made with love and Glam Metal! XOXO |" }
          <a class="footer" href="https://github.com/angeldollface/ceramic.rs">
          { "Source" }
          </a>
         </p>
        </>
    };
}