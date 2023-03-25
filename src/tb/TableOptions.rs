#[cfg(feature = "TbTableOptions")]
use leptos::*;
#[cfg(feature = "TbTableOptions")]
///This icon requires the feature `TbTableOptions` to be enabled.
#[component]
pub fn TableOptions(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-table-options" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 12v-6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 10h16" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 4v9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5.281 18.5m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M5.281 15v1.5" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M5.281 20.5v1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.312 16.75l-1.299 .75" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3.55 19.5l-1.3 .75" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M2.25 16.75l1.3 .75" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7.013 19.5l1.3 .75" /> < title > { title } < /
        title > < / svg >
    }
}
