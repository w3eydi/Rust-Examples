use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_and_show(a: i32, b: i32) -> Result<(), JsValue> {
    let window = web_sys::window().expect("there is no global 'window'");
    let document = window.document().expect("no document");
    let body = document.body().expect("document should has a body");
    let element = document.create_element("p")?;
    element.set_text_content(Some(&format!("{a} + {b} = {}", a + b)));

    body.append_child(&element)?;
    Ok(())
}
