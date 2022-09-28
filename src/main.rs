use yew::prelude::*;
use web_sys::{HtmlInputElement};
use rand::Rng;
use yew_hooks::*;
use std::env;

struct Model {
    value: i64,
    intensity: i64
}

fn roll(state:&UseStateHandle<Model>, sound:&UseMediaHandle, input_ref:&NodeRef){
    let input = input_ref.cast::<HtmlInputElement>().unwrap();

    let mut state_value = state.value;
    let mut state_intensity = state.intensity;
    let value = input.value().parse::<i64>();

    if let Ok(value) = value {
        if value != state_value  {
            state_value = num::clamp(value, 1, std::i64::MAX);
            state_intensity = 1;
        }
    }

    if state_value == 1 {
        return
    }

    let num = rand::thread_rng().gen::<f64>();
    let roll = (num * state_value as f64).ceil() as i64;
    
    let state_value = roll;
    if roll == 1 {
        sound.play();
        state_intensity = 1;
    } else {
        state_intensity = state_intensity + 1;
    }

    state.set(Model {
        value: state_value,
        intensity: state_intensity
    });
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value: 69,
        intensity: 1
    });

    let node_audio = use_node_ref();
    let src = env!("LOSS_SOUND_EFFECT_URL");
    let sound = use_media(node_audio.clone(), src.to_owned());
    let input_ref = use_node_ref();

    let onclick = {
        let state = state.clone();
        let sound = sound.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            roll(&state, &sound, &input_ref);
        })
    };

    let keydown = {
        let state = state.clone();
        let sound = sound.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                roll(&state, &sound, &input_ref);
            }
        })
    };


    html!{
        <body class={if state.value == 1 {classes!("background")} else {classes!()}}>
            <input ref={input_ref} onkeydown={keydown} value={state.value.to_string()}/>
            <div class="flexbox" onclick={onclick}>
                <audio ref={node_audio} preload="auto"/>
                <p class={if state.value == 1 {classes!("result","one")} else {classes!("result")}} style={format!("color:red;font-size: {:?}px;",20 * state.intensity)}>{state.value}</p>
            </div>  
        </body>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
