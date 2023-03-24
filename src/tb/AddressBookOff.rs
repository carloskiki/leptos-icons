#[cfg(feature = "TbAddressBookOff")]
use leptos::*;
#[cfg(feature = "TbAddressBookOff")]
///This icon requires the feature `TbAddressBookOff` to be enabled.
#[component]
pub fn AddressBookOff(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-address-book-off" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 4h10a2 2 0 0 1 2 2v10m-.57 3.399c-.363 .37 -.87 .601 -1.43 .601h-10a2 2 0 0 1 -2 -2v-12"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10 16h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 11a2 2 0 0 0 2 2m2 -2a2 2 0 0 0 -2 -2" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M4 8h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 12h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 16h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
