use istanbul_oxi_instrument::FileCoverage;
use istanbul_oxi_instrument::COVERAGE_MAGIC_KEY;
use istanbul_oxi_instrument::COVERAGE_MAGIC_VALUE;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageMagicValue {
    key: String,
    value: String,
}

#[wasm_bindgen(js_name = "getCoverageMagicConstants")]
pub fn get_coverage_magic_constants() -> JsValue {
    serde_wasm_bindgen::to_value(&CoverageMagicValue {
        key: COVERAGE_MAGIC_KEY.to_string(),
        value: COVERAGE_MAGIC_VALUE.to_string(),
    })
    .unwrap()
}

/// Wraps FileCoverage for the wasm-bindgen to allow to use oxi-coverage in JS context
/// without oxi-coverage requires wasm-bindgen directly.
#[wasm_bindgen]
pub struct FileCoverageInterop {
    inner: FileCoverage,
}

#[wasm_bindgen]
impl FileCoverageInterop {
    #[wasm_bindgen(constructor)]
    pub fn new(val: &JsValue) -> FileCoverageInterop {
        let inner: FileCoverage = val.into_serde().unwrap();

        FileCoverageInterop { inner }
    }

    #[wasm_bindgen(js_name = "getLineCoverage")]
    pub fn get_line_coverage(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.get_line_coverage()).unwrap()
    }

    #[wasm_bindgen(js_name = "f")]
    pub fn get_f(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.f).unwrap()
    }

    #[wasm_bindgen(js_name = "b")]
    pub fn get_b(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.b).unwrap()
    }

    #[wasm_bindgen(js_name = "bT")]
    pub fn get_b_t(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.b_t).unwrap()
    }

    #[wasm_bindgen(js_name = "s")]
    pub fn get_s(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.s).unwrap()
    }

    #[wasm_bindgen(js_name = "inputSourceMap")]
    pub fn get_source_map(&self) -> JsValue {
        // Not implemented yet
        JsValue::undefined()
    }
}
