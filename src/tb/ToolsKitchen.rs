#[cfg(feature = "TbToolsKitchen")]
use leptos::*;
#[cfg(feature = "TbToolsKitchen")]
///This icon requires the feature `TbToolsKitchen` to be enabled.
#[component]
pub fn ToolsKitchen(
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
        "icon icon-tabler icon-tabler-tools-kitchen" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 3h8l-1 9h-6z" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M7 18h2v3h-2z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 3v12h-5c-.023 -3.681 .184 -7.406 5 -12z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M20 15v6h-1v-3" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M8 12l0 6" /> < title > { title } < / title >
        < / svg >
    }
}
