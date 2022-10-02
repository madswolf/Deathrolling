use yew::prelude::*;
use web_sys::{HtmlInputElement};
use rand::{Rng, seq::SliceRandom};
use yew_hooks::*;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Colors<'a> {
    background: &'a str,
    result: &'a str
}

struct Model<'a> {
    value: i64,
    intensity: i64,
    colors: Colors<'a>
}
impl<'a> Colors<'a> {
    pub const fn new(background:&'a str, result:&'a str) -> Self{
        Self {background, result}
    }
}

const BACKGROUNDS: &'static [Colors] = &[
    Colors::new("#402E2A", "#FFA62B"),
    Colors::new("#F3DE8A", "#2A2B2A"),
    Colors::new("#EB9486", "#233D4D"),
    Colors::new("#5299D3", "#F2DFD7"),
    Colors::new("#FEE1C7", "#684E32")
];

fn parse_or_default(state:&UseStateHandle<Model>, input_ref:&NodeRef) -> (i64,i64) {
    return input_ref
        .cast::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<i64>()
        .map_or_else(
        |_| (state.value, state.intensity), //default 
         |x| (num::clamp(x, 1, std::i64::MAX),1)) //happy path
}


fn roll(state:&UseStateHandle<Model>, sound:&UseMediaHandle, input_ref:&NodeRef){

    let (parsed_value, post_parse_intensity) = parse_or_default(state, input_ref);

    if parsed_value == 1 {
        return
    }

    let (new_value, new_intensity) = {
        let roll = (rand::thread_rng().gen::<f64>() * parsed_value as f64).ceil() as i64;
        if roll == 1 {
            sound.play();
            (roll,1)
        } else {
            (roll, post_parse_intensity + 1)
        }   
    };

    let color: Colors = if new_value != 31 {
        BACKGROUNDS
        .iter()
        .cloned()
        .filter(|x|  {x.clone() != state.colors})
        .collect::<Vec<_>>()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_owned()
    } else {
        Colors::new("#5d198a","#CDC392")
    };

    state.set(Model {
        value: new_value,
        intensity: new_intensity,
        colors:color
    });
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value: 69,
        intensity: 1,
        colors:BACKGROUNDS.first().unwrap().to_owned()
    });

    let node_audio = use_node_ref();
    let src = dotenv_codegen::dotenv!("LOSS_SOUND_EFFECT_URL");
    let sound = use_media(node_audio.clone(), src.to_owned());
    let input_ref = use_node_ref();

    let onclick = {
        let state = state.clone();
        let sound = sound.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |_:MouseEvent| {
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

    let body_classes = if state.value == 1 {classes!("shake")} else {classes!()};
    let body_style = format!("background-color: {};", state.colors.background);

    let input_classes = if state.value == 1 {classes!("one")} else {classes!()};
    let input_style = format!("color:{};font-size: {:?}px;",state.colors.result, 20 * state.intensity + 30);

    html!{
        <body style={body_style} class={body_classes}>
            <div class="flexbox" onclick={onclick}>
                <input type="number" class={input_classes} style={input_style} ref={input_ref} onkeydown={keydown} value={state.value.to_string()}/>
                <audio ref={node_audio} preload="auto"/>
            </div>  
        </body>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
