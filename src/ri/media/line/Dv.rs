#[cfg(feature = "RiMediaLineDv")]
use leptos::*;
#[cfg(feature = "RiMediaLineDv")]
///This icon requires the feature `RiMediaLineDv` to be enabled.
#[component]
pub fn Dv(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M11.608 3H21a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1h-7v-2h6V5h-6.255A6.968 6.968 0 0 1 15 9a6.992 6.992 0 0 1-3 5.745V21a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1v-6.255A7 7 0 1 1 11.608 3zM6 13.584V20h4v-6.416a5.001 5.001 0 1 0-4 0zM8 12a3 3 0 1 1 0-6 3 3 0 0 1 0 6zm0-2a1 1 0 1 0 0-2 1 1 0 0 0 0 2zm9-3h2v2h-2V7zM7 17h2v2H7v-2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
