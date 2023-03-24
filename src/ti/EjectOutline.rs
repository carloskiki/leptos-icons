#[cfg(feature = "TiEjectOutline")]
use leptos::*;
#[cfg(feature = "TiEjectOutline")]
///This icon requires the feature `TiEjectOutline` to be enabled.
#[component]
pub fn EjectOutline(
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
        "M16 8.551v-1.551c0-1.105-.896-2-2-2h-10v10c0 1.104.896 2 2 2h1.066c.893 2.887 3.646 5 6.809 5 3.859 0 7.062-3.016 7.062-6.875.001-3.161-2.068-5.74-4.937-6.574zm-8 1.862v3.243c0 .552-.448 1-1 1s-1-.448-1-1v-6.656h6.656c.552 0 1 .448 1 1s-.448 1-1 1h-3.243l4.801 4.799c.392.391.392 1.025.001 1.415-.189.188-.439.292-.708.292-.268 0-.519-.104-.707-.291l-4.8-4.802zm6 9.618c-2.757 0-5-2.26-5-5.016 0-.705-.004-1.371.21-1.979l2.883 2.884c.39.39.901.584 1.414.584s1.022-.194 1.414-.584c.781-.78.781-2.049 0-2.83l-2.567-2.567c.517-.226 1.065-.398 1.646-.398 2.757 0 5 2.182 5 4.938 0 2.757-2.243 4.968-5 4.968z"
        /> < title > { title } < / title > < / svg >
    }
}
