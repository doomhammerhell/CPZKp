use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, Window};
use monaco::api::CodeEditor;
use cpzkp::{KeyPair, Proof};
use serde_json::{json, to_string_pretty};

#[wasm_bindgen]
pub struct Playground {
    editor: CodeEditor,
    keypair: Option<KeyPair>,
    proof: Option<Proof>,
}

#[wasm_bindgen]
impl Playground {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let editor_div = document.create_element("div").unwrap();
        editor_div.set_id("editor");
        document.body().unwrap().append_child(&editor_div).unwrap();

        let editor = CodeEditor::create(
            &editor_div,
            &json!({
                "value": "// Enter your JSON here\n{\n  \"group\": \"scalar\",\n  \"message\": \"Hello, World!\"\n}",
                "language": "json",
                "theme": "vs-dark",
                "automaticLayout": true,
            }),
        );

        Self {
            editor,
            keypair: None,
            proof: None,
        }
    }

    #[wasm_bindgen]
    pub fn generate_keypair(&mut self) -> Result<String, JsValue> {
        let content = self.editor.get_value();
        let input: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let group = input["group"]
            .as_str()
            .ok_or_else(|| JsValue::from_str("Missing group type"))?;

        self.keypair = Some(KeyPair::new(group)
            .map_err(|e| JsValue::from_str(&e.to_string()))?);

        let json = self.keypair.as_ref().unwrap().to_json()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(to_string_pretty(&json).unwrap())
    }

    #[wasm_bindgen]
    pub fn generate_proof(&mut self) -> Result<String, JsValue> {
        let content = self.editor.get_value();
        let input: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let message = input["message"]
            .as_str()
            .ok_or_else(|| JsValue::from_str("Missing message"))?;

        if let Some(keypair) = &self.keypair {
            self.proof = Some(Proof::generate(keypair, message)
                .map_err(|e| JsValue::from_str(&e.to_string()))?);

            let json = self.proof.as_ref().unwrap().to_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))?;

            Ok(to_string_pretty(&json).unwrap())
        } else {
            Err(JsValue::from_str("Generate a keypair first"))
        }
    }

    #[wasm_bindgen]
    pub fn verify_proof(&self) -> Result<bool, JsValue> {
        if let Some(proof) = &self.proof {
            proof.verify()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        } else {
            Err(JsValue::from_str("Generate a proof first"))
        }
    }
} 