#[cfg(feature = "IoWomanSharp")]
use leptos::*;
#[cfg(feature = "IoWomanSharp")]
///This icon requires the feature `IoWomanSharp` to be enabled.
#[component]
pub fn WomanSharp(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < circle xmlns =
        "http://www.w3.org/2000/svg" cx = "255.75" cy = "56" r = "56" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M310.28,191.4h.05l7.66-2.3,36.79,122.6,46-13.8-16.21-54.16c0-.12,0-.24-.07-.36l-16.84-56.12-4.71-15.74h0l-.9-3H362l-2.51-8.45a44.84,44.84,0,0,0-43-32.08H195.24a44.84,44.84,0,0,0-43,32.08l-2.51,8.45h-.06l-.9,3h0l-4.71,15.74-16.84,56.12c0,.12,0,.24-.07.36L110.94,297.9l46,13.8L193.7,189.1l7.54,2.26L148.25,368h51.5V512h52V368h8V512h52V368h51.51Z"
        /> < title > { title } < / title > < / svg >
    }
}
