#[cfg(feature = "SiChainlink")]
use leptos::*;
#[cfg(feature = "SiChainlink")]
///This icon requires the feature `SiChainlink` to be enabled.
#[component]
pub fn Chainlink(
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
        "M12 0L9.798 1.266l-6 3.468L1.596 6v12l2.202 1.266 6.055 3.468L12.055 24l2.202-1.266 5.945-3.468L22.404 18V6l-2.202-1.266-6-3.468zM6 15.468V8.532l6-3.468 6 3.468v6.936l-6 3.468z"
        /> < title > { title } < / title > < / svg >
    }
}
