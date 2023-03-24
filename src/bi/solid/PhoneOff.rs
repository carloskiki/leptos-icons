#[cfg(feature = "BiSolidPhoneOff")]
use leptos::*;
#[cfg(feature = "BiSolidPhoneOff")]
///This icon requires the feature `BiSolidPhoneOff` to be enabled.
#[component]
pub fn PhoneOff(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M9.17 13.42a5.24 5.24 0 0 1-.93-2.06L10.7 9a1 1 0 0 0 0-1.39l-3.65-4.1a1 1 0 0 0-1.4-.08L3.48 5.29a1 1 0 0 0-.29.65 15.25 15.25 0 0 0 3.54 9.92l-4.44 4.43 1.42 1.42 18-18-1.42-1.42zm7.44.02a1 1 0 0 0-1.39.05L12.82 16a4.07 4.07 0 0 1-.51-.14l-2.66 2.61A15.46 15.46 0 0 0 17.89 21h.36a1 1 0 0 0 .65-.29l1.86-2.17a1 1 0 0 0-.09-1.39z"
        /> < title > { title } < / title > < / svg >
    }
}
