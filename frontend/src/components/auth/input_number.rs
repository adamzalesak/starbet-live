use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew::prelude::*;


#[derive(Clone, PartialEq)]
pub enum NumberType {
    Ratio,
    Id,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub number_type: NumberType,
    pub label: String,
    #[prop_or_default]
    pub placeholder: String,
    // pub value: String,
    pub on_change: Callback<(f32, bool)>,
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
#[function_component(NumberInput)]
pub fn text_input(props: &Props) -> Html {
    let Props {
        number_type,
        label,
        placeholder,
        // value,
        on_change,
        required,
    } = props.clone();

    let label_copy = label.clone();
    let label_copy2 = label.clone();
    let number_type_copy = number_type.clone();

    let error_msg = use_state(String::new);
    let error_msg_copy = error_msg.clone();
    let error_msg_copy2 = (*error_msg).to_string();

    let was_touched = use_state(|| false);
    let was_touched_copy = was_touched.clone();

    let is_valid = use_state(|| false);
    let is_valid_copy = is_valid.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        let temp_value = get_value_from_input_event(input_event);
        let result: f32;
       
        match number_type_copy {
            NumberType::Ratio => {
                match temp_value.parse::<f32>() {
                    Ok(value) => {
                        result = value;
                    }
                    _ => {
                        result = 0.0;
                    },
                };
            }
            NumberType::Id => {
                match temp_value.parse::<u32>() {
                    Ok(value) => {
                        result = value as f32;
                    },
                    _ => {
                        result = 0.0
                    }
                };
            }
        };
        let flag = match required {
            false => true,
            _ => result >= 1.0,
        };

        if flag || !(*was_touched_copy) {
            error_msg.set(String::new());
        } else {
            error_msg.set(format!("Enter a valid {}", &label_copy.to_lowercase()));
        }

        is_valid.set(flag);
        on_change.emit((result, flag));
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

            <label for={format!("{:?}", label.clone())} class={format!("font-medium mb-1 {}", if required {"required_input"} else {""})}>
                {label.clone()}{":"}
            </label>

            <input
                id={format!("{:?}", label.clone())}
                type="number"
                step={format!("{}", if number_type == NumberType::Id {"1".to_string()} else {"0.01".to_string()})}
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
