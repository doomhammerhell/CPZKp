use yew::prelude::*;
use cpzkp::{Group, KeyPair, Proof};
use serde_json::to_string_pretty;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct App {
    keypair: Option<KeyPair>,
    proof: Option<Proof>,
    message: String,
}

pub enum Msg {
    GenerateKeyPair,
    GenerateProof,
    VerifyProof,
    UpdateMessage(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            keypair: None,
            proof: None,
            message: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GenerateKeyPair => {
                self.keypair = Some(KeyPair::new("scalar").unwrap());
                true
            }
            Msg::GenerateProof => {
                if let Some(keypair) = &self.keypair {
                    self.proof = Some(Proof::generate(keypair, &self.message).unwrap());
                }
                true
            }
            Msg::VerifyProof => {
                if let Some(proof) = &self.proof {
                    let is_valid = proof.verify().unwrap();
                    log(&format!("Proof verification: {}", is_valid));
                }
                true
            }
            Msg::UpdateMessage(msg) => {
                self.message = msg;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <h1>{ "CPZKp Web Demo" }</h1>
                
                <div class="section">
                    <h2>{ "Key Pair" }</h2>
                    <button onclick={ctx.link().callback(|_| Msg::GenerateKeyPair)}>
                        { "Generate Key Pair" }
                    </button>
                    if let Some(keypair) = &self.keypair {
                        <pre>
                            { to_string_pretty(&keypair.to_json().unwrap()).unwrap() }
                        </pre>
                    }
                </div>

                <div class="section">
                    <h2>{ "Proof" }</h2>
                    <input
                        type="text"
                        value={self.message.clone()}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            Msg::UpdateMessage(input.value())
                        })}
                        placeholder="Enter message"
                    />
                    <button onclick={ctx.link().callback(|_| Msg::GenerateProof)}>
                        { "Generate Proof" }
                    </button>
                    <button onclick={ctx.link().callback(|_| Msg::VerifyProof)}>
                        { "Verify Proof" }
                    </button>
                    if let Some(proof) = &self.proof {
                        <pre>
                            { to_string_pretty(&proof.to_json().unwrap()).unwrap() }
                        </pre>
                    }
                </div>
            </div>
        }
    }
} 