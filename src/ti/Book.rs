#[cfg(feature = "TiBook")]
use leptos::*;
#[cfg(feature = "TiBook")]
///This icon requires the feature `TiBook` to be enabled.
#[component]
pub fn Book(
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
        "M18 3h-11c-.265 0-.52.105-.707.293l-3 3-.057.062c-.139.165-.225.373-.235.6l-.001.053v10.992c0 1.654 1.346 3 3 3h9c1.304 0 2.416-.836 2.829-2h.671c1.402 0 2.5-1.317 2.5-3v-10c0-1.654-1.346-3-3-3zm-12 16c-.551 0-1-.448-1-1v-10h2v11h-1zm10-1c0 .552-.449 1-1 1h-7v-11h7c.551 0 1 .448 1 1v9zm3-2c0 .62-.324 1-.5 1h-.5v-8c0-1.654-1.346-3-3-3h-8.586l1-1h10.586c.551 0 1 .448 1 1v10z"
        /> < title > { title } < / title > < / svg >
    }
}
