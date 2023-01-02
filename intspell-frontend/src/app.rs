use chrono::Datelike;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use yew::prelude::*;


#[function_component(App)]
pub fn app() -> Html {
    let oai_key: UseStateHandle<AttrValue> = use_state_eq(|| AttrValue::from(""));

    let set_oai_key: Callback<AttrValue> = {
        let oai_key = oai_key.clone();

        Callback::from(move |val: AttrValue| {
            oai_key.set(val)
        })
    };

    let result: Html = html! {<>
        <header>
            <h1>
                {"Steffo's Intelligent Spellbook"}
            </h1>
        </header>
        <main>
            <div class="chapter-1">
                <Configurator oai_key={(*oai_key).clone()} set_oai_key={set_oai_key}/>
            </div>
            <div class="chapter-1">
                <Generator oai_key={(*oai_key).clone()}/>
            </div>
        </main>
        <footer>
            <span>
                {"© "}
                {chrono::offset::Local::now().year().to_string()}
                {" Stefano Pigozzi"}
            </span>
            {" | "}
            <a href="">
                {"Source code"}
            </a>
        </footer>
    </>};

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
            <h3>
                {"Configuration"}
            </h3>
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

    let result: Html = html! {
        <form class={"panel box form-flex"}>
            <h3>
                {"Spell generator"}
            </h3>
            <label>
                <span>{"Spell Name"}</span>
                <input type="text" placeholder="Tiny Hat"/>
                <span/>
            </label>
            <label class={classes!(disabled.then_some("fade"))}>
                <span>{"Generate"}</span>
                <button {disabled}>
                    {"with your OpenAI API key"}
                </button>
                <span/>
            </label>
            <label class={"fade"}>
                <span>{"Generate"}</span>
                <button disabled={true}>
                    {"using Intelligent Spellbook"}
                </button>
                <span/>
            </label>
        </form>
    };

    result
}


#[derive(Properties, PartialEq)]
pub struct VisualizerProps {
    pub text: String,
}

#[function_component(Visualizer)]
pub fn visualizer(VisualizerProps{text}: &VisualizerProps) -> Html {
    let result = html! {
        <article class="panel spell">
            {"Hello world!"}
        </article>
    };

    result
}