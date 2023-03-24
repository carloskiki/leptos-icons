#[cfg(feature = "SiAlacritty")]
use leptos::*;
#[cfg(feature = "SiAlacritty")]
///This icon requires the feature `SiAlacritty` to be enabled.
#[component]
pub fn Alacritty(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "m10.065 0-8.57 21.269h3.595l6.91-16.244 6.91 16.244h3.594l-8.57-21.269zm1.935 9.935c-0.76666 1.8547-1.5334 3.7094-2.298 5.565 1.475 4.54 1.475 4.54 2.298 8.5 0.823-3.96 0.823-3.96 2.297-8.5-0.76637-1.8547-1.5315-3.7099-2.297-5.565z"
        /> < title > { title } < / title > < / svg >
    }
}
