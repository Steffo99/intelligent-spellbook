use chrono::Datelike;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use crate::callback;


#[function_component(App)]
pub fn app() -> Html {
    let key: UseStateHandle<AttrValue> = use_state_eq(|| AttrValue::from(""));

    let set_key = callback! {
        |val: AttrValue| => {
            key.set(val);
        },
        [key]
    };

    let result: Html = html! {<>
        <Header/>
        <main>
            <div class="chapter-1">
                <Configurator oai_key={(*key).clone()} set_oai_key={set_key}/>
            </div>
            <div class="chapter-1">
                <Generator oai_key={(*key).clone()}/>
            </div>
        </main>
        <Footer/>
    </>};

    result
}

#[function_component(Header)]
pub fn header() -> Html {
    let result = html! {
        <header>
            <h1>
                {"Steffo's Intelligent Spellbook"}
            </h1>
        </header>
    };

    result
}

#[function_component(Footer)]
pub fn footer() -> Html {
    let result = html! {
        <footer>
            <span>
                {"© "}
                <ThisYear/>
                {" Stefano Pigozzi"}
            </span>
            {" | "}
            <span>
                {"Powered by "}
                <a href="https://openai.com/api/">
                    {"OpenAI text-davinci-003"}
                </a>
            </span>
            {" | "}
            <a href="">
                {"Source code"}
            </a>
        </footer>
    };

    result
}

#[function_component(ThisYear)]
pub fn this_year() -> Html {
    let now = chrono::offset::Local::now();

    let result: Html = html! {
        <time datetime={now.timestamp().to_string()}>
            {now.year().to_string()}
        </time>
    };

    result
}


#[derive(Properties, PartialEq)]
pub struct ConfiguratorProps {
    pub oai_key: AttrValue,
    pub set_oai_key: Callback<AttrValue>,
}

#[function_component(Configurator)]
pub fn configurator(ConfiguratorProps{oai_key, set_oai_key}: &ConfiguratorProps) -> Html {

    let oninput_oai_key = {
        let set_oai_key = set_oai_key.clone();

        Callback::from(move |event: InputEvent| {
            let value: AttrValue = match event.target() {
                Some(target) => unsafe {
                    target.unchecked_into::<HtmlInputElement>().value().into()
                },
                None => AttrValue::from(""),
            };
            set_oai_key.emit(value);
        })
    };

    let result: Html = html! {
        <form class={"panel box form-flex"}>
            <h2>
                {"Configuration"}
            </h2>
            <label>
                <span>{"OpenAI API key"}</span>
                <input
                    type="password"
                    placeholder="sk-...wxyz"
                    value={oai_key}
                    oninput={oninput_oai_key}
                />
                <span/>
            </label>
        </form>
    };

    result
}


#[derive(Properties, PartialEq)]
pub struct GeneratorProps {
    pub oai_key: AttrValue,
}

#[function_component(Generator)]
pub fn generator( GeneratorProps{ oai_key }: &GeneratorProps) -> Html {
    let disabled = !oai_key.starts_with("sk-");

    let sample_text = AttrValue::from(r#"
        Gelato
        2nd-level Transmutation (Arcane)

        Casting Time: 1 action
        Range: 30 feet
        Components: V, S, M (a scoop of gelato)
        Duration: Instantaneous

        You transform a scoop of gelato in your hand into a magical force that affects the creatures of your choice within range. Choose up to three creatures within range. Each target must make a Dexterity saving throw. On a failed save, a target takes 1d8 cold damage, and their speed is reduced by 10 feet until the start of your next turn. On a successful save, a target takes half as much damage, and their speed is not reduced.

        At Higher Levels. When you cast this spell using a spell slot of 3rd level or higher, the damage increases by 1d8 for each slot level above 2nd.
    "#);

    let result: Html = html! {
        <form class={"panel box form-flex"}>
            <h2>
                {"Spell generator"}
            </h2>
            <label>
                <span>{"Spell Name"}</span>
                <input type="text" placeholder="Tiny Hat"/>
                <span/>
            </label>
            <label class={classes!(disabled.then_some("fade"))}>
                <span>{"Generate"}</span>
                <button type="button" {disabled}>
                    {"with your OpenAI API key"}
                </button>
                <span/>
            </label>
            <Visualizer text={sample_text}/>
        </form>
    };

    result
}


#[derive(Properties, PartialEq)]
pub struct VisualizerProps {
    pub text: AttrValue,
}

#[function_component(Visualizer)]
pub fn visualizer(VisualizerProps{text}: &VisualizerProps) -> Html {
    let lines = text.trim().split("\n").map(|s| s.to_owned()).collect::<Vec<String>>();
    let (title, rest) = lines.split_at(1);

    let title = match title.get(0) {
        None => html!{ <h3 class="title"/> },
        Some(title) => html!{ <h3 class="title">{title}</h3> },
    };

    let rest = rest.iter().map(|l| {
        html! {
            <VisualizerLine text={l.to_owned()}/>
        }
    }).collect::<Html>();

    let result = html! {
        <article class="panel spell">
            {title}
            {rest}
        </article>
    };

    result
}


#[derive(Properties, PartialEq)]
pub struct VisualizerLineProps {
    pub text: String,
}

#[function_component(VisualizerLine)]
pub fn visualizer_line(VisualizerLineProps{text}: &VisualizerLineProps) -> Html {
    if let Some((dt, dl)) = text.split_once(": ") {
        html! {
            <p>
                <b>{dt}</b>{": "}<span>{dl}</span>
            </p>
        }
    }
    else {
        html! {
            <p>
                {text}
            </p>
        }
    }
}