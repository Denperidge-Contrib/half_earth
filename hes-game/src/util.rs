use base64::prelude::*;
use extend::ext;
use hes_engine::flavor::{Image, ImageData};
use html::ElementDescriptor;
use leptos::{wasm_bindgen::JsCast, *};
use leptos_use::use_window;
use web_sys::{HtmlCollection, NodeList};

/// Iteratively scale text (by decreasing the font size) until it fits
/// or reaches the `min_size`.
pub fn scale_text(elem: web_sys::HtmlElement, min_size: u32) {
    if let Some(initial_font_size) = get_font_size(&elem) {
        let mut font_size = initial_font_size as u32;
        while (elem.scroll_height() > elem.client_height()
            || elem.scroll_width() > elem.client_width())
            && font_size > min_size
        {
            let next_size = font_size - 1;
            let _ = elem.style().set_property(
                "font-size",
                &format!("{next_size}px"),
            );
            font_size = next_size;
        }
    }
}

/// Get the font size of a given element.
fn get_font_size(elem: &web_sys::HtmlElement) -> Option<i32> {
    window().get_computed_style(elem).unwrap().map(|style| {
        style
            .get_property_value("font-size")
            .unwrap()
            .replace("px", "")
            .parse::<f32>()
            .unwrap_or(15.)
            .round() as i32
    })
}

/// Guess if this browser is a Safari browser.
pub fn is_safari() -> bool {
    let window = use_window();
    window
        .navigator()
        .map(|navigator| {
            if let Ok(agent) = navigator.user_agent() {
                agent.contains("Safari")
                    && !agent.contains("Chrome")
            } else {
                false
            }
        })
        .unwrap_or_default()
}

/// Guess if we're running on Steam.
pub fn is_steam() -> bool {
    std::option_env!("PLATFORM") != Some("STEAM")
}

/// Identify the index of the child in the center
/// of this element.
pub fn detect_center_element(
    parent: web_sys::HtmlElement,
    elements: &[web_sys::HtmlElement],
) -> Option<usize> {
    let rect = parent.get_bounding_client_rect();
    let target_x = rect.x() + parent.client_width() as f64 / 2.;
    let mut min_dist = f64::INFINITY;
    let mut closest = None;

    for (idx, element) in elements.iter().enumerate() {
        let rect = element.get_bounding_client_rect();
        let pos = rect.x() + rect.width() / 2.;
        let dist = (target_x - pos).abs();
        if dist < min_dist {
            min_dist = dist;
            closest = Some(idx);
        }
    }
    closest
}

/// Convert a `NodeList` to a vec of elements.
pub fn nodelist_to_elements(
    nodelist: web_sys::NodeList,
) -> Vec<web_sys::HtmlElement> {
    (0..nodelist.length())
        .filter_map(|i| nodelist.item(i))
        .filter_map(|node| {
            node.dyn_into::<web_sys::HtmlElement>().ok()
        })
        .collect()
}

/// Convert an `HtmlCollection` to a vec of elements.
pub fn collection_to_elements(
    collection: HtmlCollection,
) -> Vec<web_sys::HtmlElement> {
    let mut elements = Vec::new();
    for i in 0..collection.length() {
        if let Some(element) = collection.item(i) {
            if let Ok(html_element) =
                element.dyn_into::<web_sys::HtmlElement>()
            {
                elements.push(html_element);
            }
        }
    }
    elements
}

/// Convert from a `leptos::HtmlElement` to a `web_sys::HtmlElement`.
pub fn to_ws_el<T: ElementDescriptor + 'static>(
    el: HtmlElement<T>,
) -> web_sys::HtmlElement {
    let el = el.into_any();
    let el: &web_sys::HtmlElement = el.as_ref();
    el.clone()
}

/// Adjust card scale depending on the screen height.
pub fn card_scale() -> f32 {
    let height = use_window()
        .document()
        .body()
        .expect("Will have a body client-side")
        .client_height();
    if height < 600 {
        0.9
    } else {
        1.0
    }
}

#[ext]
pub impl Image {
    fn src(&self) -> String {
        match &self.data {
            ImageData::File(fname) => {
                format!("/assets/content/images/{fname}",)
            }
            ImageData::Data { bytes, mime } => format!(
                "data:{mime};charset=utf-8;base64,{}",
                BASE64_STANDARD.encode(bytes)
            ),
        }
    }
}

pub fn get_element(id: &str) -> web_sys::HtmlElement {
    // TODO: use a shared document instance perhaps? This feels dirty
    // But this is the first Rust code I've written so I don't know how to do that
    let document = web_sys::window().unwrap().document().unwrap();
    return document.get_element_by_id(id).unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
}

/*
pub fn get_dialog(id: &str) -> web_sys::HtmlDialogElement {
    let document = web_sys::window().unwrap().document().unwrap();
    return document.get_element_by_id(id).unwrap().dyn_into::<web_sys::HtmlDialogElement>().unwrap();
}
     */

const FOCUSSABLE_ELEMENTS_IN: &str = "#{id} a,#{id} area,#{id} button,#{id} frame,#{id} iframe,#{id} input,#{id} object,#{id} select,#{id} textarea,#{id} svg a,#{id} summary";
const FOCUSSABLE_ELEMENTS_EXCEPT: &str = "a:not(#{id}),area:not(#{id}),button:not(#{id}),frame:not(#{id}),iframe:not(#{id}),input:not(#{id}),object:not(#{id}),select:not(#{id}),textarea:not(#{id}),svg a:not(#{id}),summary:not(#{id})";


fn apply_tabindex(elements: NodeList, value: i32) {
    // Possible to do queryselectorall without dyn_into'ing everything?

    let length: u32 = elements.length();

    for i in 0..=length {
        let option = elements.get(i);
        
        if (option.is_some()) {
            option.unwrap()
                .dyn_into::<web_sys::HtmlElement>().unwrap()
                .set_tab_index(value);
        }

    }
}

pub fn tabindex_focus(id: &str, tabindex: i32, reverse_non_selected: bool) {
    let document = web_sys::window().unwrap().document().unwrap();

    let id_elements = document.query_selector_all(
        format!(
            "#{id} a,#{id} area,#{id} button,#{id} frame,#{id} iframe,#{id} input,#{id} object,#{id} select,#{id} textarea,#{id} svg a,#{id} summary", 
            id=id).as_str()).unwrap();

    apply_tabindex(id_elements, tabindex);

    if tabindex != 0 && reverse_non_selected {
        let non_id_elements = document.query_selector_all(
            format!(
                "a:not(#{id}),area:not(#{id} *),button:not(#{id} *),frame:not(#{id} *),iframe:not(#{id} *),input:not(#{id} *),object:not(#{id} *),select:not(#{id} *),textarea:not(#{id} *),svg a:not(#{id} *),summary:not(#{id} *)",
                id=id).as_str()).unwrap();
        if tabindex > 0 {
            apply_tabindex(non_id_elements, -1);
        } else {
            apply_tabindex(non_id_elements, 1);
        }
    }
}