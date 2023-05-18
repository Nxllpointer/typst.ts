use crate::TypstCompiler;

use typst_ts_compiler::font::web::BrowserFontSearcher;

use js_sys::Uint8Array;
use typst::util::Buffer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TypstCompilerBuilder {
    searcher: BrowserFontSearcher,
}

#[wasm_bindgen]
impl TypstCompilerBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<TypstCompilerBuilder, JsValue> {
        console_error_panic_hook::set_once();
        Ok(Self {
            searcher: BrowserFontSearcher::new(),
        })
    }

    // 400 KB
    pub async fn add_raw_font(&mut self, font_buffer: Uint8Array) -> Result<(), JsValue> {
        // let v: JsValue =
        //     format!("raw font loading: Buffer({:?})", font_buffer.byte_length()).into();
        // console::info_1(&v);

        self.add_raw_font_internal(font_buffer.to_vec().into());
        Ok(())
    }

    // 100 KB
    pub async fn add_web_fonts(&mut self, fonts: js_sys::Array) -> Result<(), JsValue> {
        self.searcher.add_web_fonts(fonts).await
    }

    // 24 MB
    pub async fn build(self) -> Result<TypstCompiler, JsValue> {
        TypstCompiler::new(self.searcher).await
    }
}

impl TypstCompilerBuilder {
    pub fn add_raw_font_internal(&mut self, font_buffer: Buffer) {
        self.searcher.add_font_data(font_buffer);
    }
}
