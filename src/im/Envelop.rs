#[cfg(feature = "ImEnvelop")]
use leptos::*;
#[cfg(feature = "ImEnvelop")]
///This icon requires the feature `ImEnvelop` to be enabled.
#[component]
pub fn Envelop(
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
        "M14.5 2h-13c-0.825 0-1.5 0.675-1.5 1.5v10c0 0.825 0.675 1.5 1.5 1.5h13c0.825 0 1.5-0.675 1.5-1.5v-10c0-0.825-0.675-1.5-1.5-1.5zM6.23 8.6l-4.23 3.295v-7.838l4.23 4.543zM2.756 4h10.488l-5.244 3.938-5.244-3.938zM6.395 8.777l1.605 1.723 1.605-1.723 3.29 4.223h-9.79l3.29-4.223zM9.77 8.6l4.23-4.543v7.838l-4.23-3.295z"
        /> < title > { title } < / title > < / svg >
    }
}
