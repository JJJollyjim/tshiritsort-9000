#![feature(slice_patterns)]

mod sortation;
mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let select = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("id_option1")
        .unwrap();
    let mut elems: Vec<(usize, _)> = Vec::new();

    let mut i = 0;
    while let Some(child) = select.children().item(0) {
        select.remove_child(&child)?;
        elems.push((i, child));
        i += 1;
    }

    let texts: Vec<_> = elems
        .iter()
        .map(|(_, elem)| elem.unchecked_ref::<web_sys::HtmlElement>().inner_text())
        .collect();

    // INITIATE SORTATION
    elems.sort_by_cached_key(|(i, _)| sortation::size_sort_key(&texts[*i]));

    select.set_inner_html("");
    for (_, e) in elems {
        select.append_child(&e)?;
    }

    // this is dumb.

    Ok(())
}
