/*
CERAMIC.RS by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

use yew::prelude::*;

// We define the info
// component.
#[function_component]
pub fn InfoCog () -> Html {

    // The HTML to render to the DOM.
    // Container "div" and instructions for iOS and Android.
    return html!{
        <div class="content">
         <h2 class="info">{ "How do I find my IMEI?" }</h2>
          <h3 class="info">{ "iOS" }</h3>
           <p class="info">
            { "Head over to \"Settings\", go to \"General\", and tap \"About\". 
            Near the bottom there should be an item marked \"IMEI\"." }
           </p>
           <h3 class="info">{ "Android" }</h3>
           <p class="info">
            { "Open the dialer app. Type \"*#06#\". Your IMEI will be displayed." }
           </p>
        </div>
    };
}