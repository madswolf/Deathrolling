use yew::prelude::*;
use web_sys::{HtmlInputElement, EventTarget};
use wasm_bindgen::JsCast;
use rand::Rng;

struct Model {
    value: i64,
    intensity: i64
}


fn to_html_input_element(target:Option<EventTarget>) -> HtmlInputElement{
    target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).unwrap()
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value: 0,
        intensity: 0
    });

    let node_video = use_node_ref();
    let src = "https://media.clown.mads.monster/bass2.mp3";
    let sound = yew_hooks::use_media(node_video.clone(), src.to_owned());

    fn roll(state: UseStateHandle<Model>) -> (){
        let num = rand::thread_rng().gen::<f64>();
        let roll = (num * state.value as f64).ceil() as i64;
        
        let state_value = roll;
        let mut state_intensity = state.intensity;
        if roll == 1 {
            log::info!("donorono");
        } else {
            state_intensity = state_intensity +1;
            log::info!("gaming intensity: {:?} ", state_intensity);
        }

        state.set(Model {
            value: state_value,
            intensity: state_intensity
        });
    }

    let onclick = {
        let state = state.clone();
        let sound = sound.clone();
        Callback::from(move |event:MouseEvent| {
            let num = rand::thread_rng().gen::<f64>();
            let roll = (num * state.value as f64).ceil() as i64;
            
            let state_value = roll;
            let mut state_intensity = state.intensity;
            if roll == 1 {
                sound.play();
                log::info!("donorono");
            } else {
                state_intensity = state_intensity +1;
                log::info!("gaming intensity: {:?} ", state_intensity);
            }
    
    
            log::info!("set value to {:?}",state_value);
            
            state.set(Model {
                value: state_value,
                intensity: state_intensity
            });
        })
    };

    let keydown ={
        let state = state.clone();
        let sound = sound.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Enter" {

                let input = to_html_input_element(event.target());


                let mut state_value = state.value;
                let mut state_intensity = state.intensity;
                let value = input.value().parse::<i64>();

                if let Ok(value) = value {
                    if value != state_value  {
                        state_value = value;
                        state_intensity = 1;
                    }
                }

                let num = rand::thread_rng().gen::<f64>();
                let roll = (num * state_value as f64).ceil() as i64;
                
                let state_value = roll;
                if roll == 1 {
                    sound.play();
                    state_intensity = 1;
                    log::info!("donorono");
                } else {
                    state_intensity = state_intensity +1;
                    log::info!("gaming intensity: {:?} ", state_intensity);
                }
            
                state.set(Model {
                    value: state_value,
                    intensity: state_intensity
                });
                    }
        })
    };

    html!{
        <body class={if state.value == 1 {classes!("background")} else {classes!()}}>
            <input onkeydown={keydown} value={state.value.to_string()}/>
            <div class="flexbox" onclick={onclick}>
            <audio ref={node_video}/>
            <p class={if state.value == 1 {classes!("result","one")} else {classes!("result")}} style={format!("color:red;font-size: {:?}px;",20 * state.intensity)}>{state.value}</p>
            </div>
        </body>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
