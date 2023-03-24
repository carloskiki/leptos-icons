#[cfg(feature = "TiArrowMinimise")]
use leptos::*;
#[cfg(feature = "TiArrowMinimise")]
///This icon requires the feature `TiArrowMinimise` to be enabled.
#[component]
pub fn ArrowMinimise(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.121 13c-.553 0-1 .448-1 1s.447 1 1 1h1.465l-3.293 3.293c-.391.391-.391 1.023 0 1.414.195.195.451.293.707.293s.512-.098.707-.293l3.414-3.414v1.707c0 .552.447 1 1 1s.879-.448.879-1v-5h-4.879zM7 11c.552 0 1-.448 1-1v-2h2c.553 0 1-.448 1-1s-.447-1-1-1h-3.999l-.001 4c0 .552.447 1 1 1zM17 13c-.553 0-1 .448-1 1v2h-2c-.553 0-1 .448-1 1s.447 1 1 1h4v-4c0-.552-.447-1-1-1zM18.293 4.293l-3.293 3.293v-1.586c0-.552-.447-1-1-1s-1 .448-1 1v5h5c.552 0 1-.448 1-1s-.447-1-1-1h-1.586l3.293-3.292c.391-.391.391-1.023 0-1.414s-1.023-.392-1.414-.001z"
        /> < title > { title } < / title > < / svg >
    }
}
