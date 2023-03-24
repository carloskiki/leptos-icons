#[cfg(feature = "SiPepsi")]
use leptos::*;
#[cfg(feature = "SiPepsi")]
///This icon requires the feature `SiPepsi` to be enabled.
#[component]
pub fn Pepsi(
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
        "M18.43 3.277A10.839 10.839 0 002.718 17.594c7.455-2.033 13.503-7 15.712-14.317M12 22.84a10.839 10.839 0 009.21-16.574 7.607 7.607 0 01-2.873 8.195c-3.285 2.416-8.06 2.432-14.649 4.494A10.817 10.817 0 0012 22.84M24 12A12 12 0 1112 0a12 12 0 0112 12"
        /> < title > { title } < / title > < / svg >
    }
}
