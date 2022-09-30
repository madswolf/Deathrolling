use yew::prelude::*;
use web_sys::{HtmlInputElement};
use rand::{Rng, seq::SliceRandom};
use yew_hooks::*;
use std::env;

#[derive(Eq, PartialEq, Clone)]
struct Colors {
    background: String,
    result: String
}

struct Model {
    value: i64,
    intensity: i64,
    colors: Colors
}


fn roll(state:&UseStateHandle<Model>, sound:&UseMediaHandle, input_ref:&NodeRef){
    let input = input_ref.cast::<HtmlInputElement>().unwrap();

    let backgrounds:Vec<Colors> = vec![
        Colors{background:"#52AA5E".to_owned(),result:"#007991".to_owned()},
        Colors{background:"#F3DE8A".to_owned(),result:"#2A2B2A".to_owned()},
        Colors{background:"#EB9486".to_owned(),result:"#233D4D".to_owned()},
        Colors{background:"#5299D3".to_owned(),result:"#F2DFD7".to_owned()},
        Colors{background:"#FEE1C7".to_owned(),result:"#684E32".to_owned()}
    ];

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
    
    let thing = backgrounds.iter().cloned().filter(|x|  {x.clone() != state.colors}).collect::<Vec<_>>();
    let Colors{background:mut background_color, result: result_color} = thing.choose(&mut rand::thread_rng()).unwrap().to_owned();
    log::info!("background_color:{} result_color:{}", background_color, result_color);

    if state_value == 31 {
        background_color = "#5d198a".to_owned();
    }

    state.set(Model {
        value: state_value,
        intensity: state_intensity,
        colors:Colors{background:background_color, result: result_color}
    });
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value: 69,
        intensity: 1,
        colors:Colors{background:"#52AA5E".to_owned(), result:"red".to_owned()}
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
        <body style={format!("background-color: {};", state.colors.background)} class={if state.value == 1 {classes!("background")} else {classes!()}}>
            <input ref={input_ref} onkeydown={keydown} value={state.value.to_string()}/>
            <div class="flexbox" onclick={onclick}>
                <audio ref={node_audio} preload="auto"/>
                <p class={if state.value == 1 {classes!("result","one")} else {classes!("result")}} style={format!("color:{};font-size: {:?}px;",state.colors.result, 20 * state.intensity + 30)}>{state.value}</p>
            </div>  
        </body>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
