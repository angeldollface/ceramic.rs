/*
CERAMIC.RS by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We load the data processing
// component.
use super::luhny::*;

// We load Yew's APIs.
use yew::prelude::*;

// We need this to work with
// events.
use wasm_bindgen::JsCast;

// We need this to capture event
// results.
use web_sys::EventTarget;

// We need this to interact
// with the HTML "input"
// element.
use web_sys::HtmlInputElement;

// We define the main data processing
// component.
#[function_component]
pub fn DataCog() -> Html {

    // Setting up our stateful result information holder.
    let result: UseStateHandle<String> = use_state(|| "Enter an IMEI number!".to_owned());

    // We capture the user's input in this stateful variable.
    let imei_number: UseStateHandle<String> = use_state(|| "".to_owned());

    // Defining the closure to intercept and process user input.
    let onchange = {

        // We clone our initial variable to re-use it.
        let imei_number_cloned: UseStateHandle<String> = imei_number.clone();

        // We instantiate a callback closure in which we then set "imei_number" to
        // the user's input.
        Callback::from(
            move |event: Event| {

                // We get the event's target: "input".
                let target: EventTarget = event.target().unwrap();

                // We convert it into an HTML element.
                let input: HtmlInputElement = target.unchecked_into::<HtmlInputElement>();

                // We update "imei_number" with the user's input.
                imei_number_cloned.set(input.value());
            }
        )
    };

    // Defining our closure to intercept button clicks.
    let onclick = {

        // We clone our initial variable to re-use it.
        let result_clone = result.clone();

        // We make a closure to interact with our variables
        // when a button is clicked.
        move |_| {

            // Is it a valid IMEI? We update "result" accordingly.
            if validate_IMEI(&imei_number.to_string()) {
                result_clone.set(String::from("true"));
            }

            // Did we receive empty input? We update "result" accordingly.
            else if &imei_number.to_string() == &String::from("") {
                result_clone.set(
                    format!(
                        "Empty input!"
                    )
                );
            }

            // Did we receive too lengthy input or the opposite? We update "result" accordingly.
            else if clean_split(&imei_number.to_string(), &String::from("")).len() != 15 {
                result_clone.set(
                    format!(
                        "IMEI length invalid!"
                    )
                );
            }

            // Is our input a sequence of digits? We update "result" accordingly.
            else if !is_number_sequence(&imei_number.to_string()) {
                result_clone.set(
                    format!(
                        "Illegal characters found!"
                    )
                );
            }

            // In any other case the input is invalid.
            else {
                result_clone.set(String::from("false"));
            }
        }
    };

    // The HTML to render to the DOM.
    // Main "div" container, "input", "button", and feedback.
    return html!{
        <div class="content">
         <input type="text" {onchange} placeholder={"YOUR IMEI GOES HERE."}/>
         <button {onclick}>{ "Echo" }</button>
         <p class="result">{ format!("IMEI valid: {}", &result.clone().to_string()) }</p>
        </div>
    }
}