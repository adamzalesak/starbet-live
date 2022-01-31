use std::fmt::Display;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew::prelude::*;

use crate::types::users::Field;

#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
    Email,
    Date,
    Password,
    DateTime,
}

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Date => "date",
            InputType::DateTime => "datetime-local",
        };
        write!(f, "{}", result)
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub field: Field, // form's field eg email, password....
    #[prop_or(InputType::Text)]
    pub input_type: InputType,
    pub label: String,
    #[prop_or_default]
    pub placeholder: String,
    // pub value: String,
    pub on_change: Callback<(String, Field, bool)>,
    #[prop_or(true)]
    pub required: bool,
}

// parsing function
fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

/// Controlled Text Input Component
#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props {
        field,
        input_type,
        label,
        placeholder,
        // value,
        on_change,
        required,
    } = props.clone();

    let field_copy = field.clone();
    let label_copy = label.clone();
    let label_copy2 = label.clone();
    let input_type_copy = input_type.clone();

    let error_msg = use_state(String::new);
    let error_msg_copy = error_msg.clone();
    let error_msg_copy2 = (*error_msg).to_string();

    let was_touched = use_state(|| false);
    let was_touched_copy = was_touched.clone();

    let is_valid = use_state(|| false);
    let is_valid_copy = is_valid.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        let temp_value = get_value_from_input_event(input_event);
        let flag = match required {
            false => true,
            _ => match input_type_copy {
                InputType::Email => temp_value.contains('@') && temp_value.contains('.'),
                InputType::Password => temp_value.trim().len() >= 6,
                InputType::Text => temp_value.trim().len() > 2 && temp_value.trim().len() <= 255,
                InputType::Date => !temp_value.trim().is_empty(),
                InputType::DateTime => !temp_value.trim().is_empty(),
            },
        };

        if flag || !(*was_touched_copy) {
            error_msg.set(String::new());
        } else {
            error_msg.set(format!("Enter a valid {}", &label_copy.to_lowercase()));
        }

        is_valid.set(flag);
        on_change.emit((temp_value, field.clone(), flag));
    });

    let onblur = Callback::from(move |_| {
        was_touched.set(true);
        if !(*is_valid_copy) && required {
            error_msg_copy.set(format!("Enter a valid {}", &label_copy2.to_lowercase()));
        } else {
            error_msg_copy.set(String::new());
        }
    });

    html! {
        <div class="flex flex-col mt-2 mb-1">

            <label for={format!("{:?}", field_copy)} class={format!("font-medium mb-1 {}", if required {"required_input"} else {""})}>
                {label}{":"}
            </label>

            <input
                id={format!("{:?}", field_copy)}
                type={input_type.to_string()}
                // {value}
                {oninput}
                {onblur}
                {placeholder}
                class={format!("border border-dark-blue focus:outline-none p-1 rounded-md shadow-md {}",
                            if !error_msg_copy2.is_empty() {"bg-danger-light border-danger"} else {""})}
            />
            {
                if !(*error_msg_copy2).is_empty() {
                    html! { <div class="text-sm text-danger">{ error_msg_copy2.to_string() }</div> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
