#[cfg(feature = "IoTerminalSharp")]
use leptos::*;
#[cfg(feature = "IoTerminalSharp")]
///This icon requires the feature `IoTerminalSharp` to be enabled.
#[component]
pub fn TerminalSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M16,44V468a12,12,0,0,0,12,12H484a12,12,0,0,0,12-12V44a12,12,0,0,0-12-12H28A12,12,0,0,0,16,44ZM73.51,237.5,150.39,176,73.51,114.5l20-25L201.61,176,93.5,262.49ZM272,256H176V224h96Z"
        /> < title > { title } < / title > < / svg >
    }
}
