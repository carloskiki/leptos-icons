#[cfg(feature = "ImMap2")]
use leptos::*;
#[cfg(feature = "ImMap2")]
///This icon requires the feature `ImMap2` to be enabled.
#[component]
pub fn Map2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M10.5 3l-5-2-5.5 2v12l5.5-2 5 2 5.5-2v-12l-5.5 2zM6 2.277l4 1.6v9.846l-4-1.6v-9.846zM1 3.7l4-1.455v9.872l-4 1.454v-9.872zM15 12.3l-4 1.455v-9.872l4-1.455v9.872z"
        /> < title > { title } < / title > < / svg >
    }
}
