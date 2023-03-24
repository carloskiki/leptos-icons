#[cfg(feature = "TiPower")]
use leptos::*;
#[cfg(feature = "TiPower")]
///This icon requires the feature `TiPower` to be enabled.
#[component]
pub fn Power(
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
        "M11.5 18.573c-1.736 0-3.368-.676-4.596-1.903-1.227-1.228-1.904-2.86-1.904-4.597s.677-3.369 1.904-4.597c.391-.391 1.023-.391 1.414 0s.391 1.023 0 1.414c-.85.851-1.318 1.981-1.318 3.183s.468 2.333 1.318 3.183c.85.85 1.979 1.317 3.182 1.317s2.332-.468 3.182-1.317c.851-.85 1.318-1.98 1.318-3.183s-.468-2.333-1.318-3.183c-.391-.391-.391-1.023 0-1.414s1.023-.391 1.414 0c1.227 1.229 1.904 2.861 1.904 4.597s-.677 3.369-1.904 4.597c-1.228 1.227-2.86 1.903-4.596 1.903zM11.5 11c-.553 0-1-.448-1-1v-5c0-.552.447-1 1-1s1 .448 1 1v5c0 .552-.447 1-1 1z"
        /> < title > { title } < / title > < / svg >
    }
}
