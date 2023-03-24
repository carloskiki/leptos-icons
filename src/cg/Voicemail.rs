#[cfg(feature = "CgVoicemail")]
use leptos::*;
#[cfg(feature = "CgVoicemail")]
///This icon requires the feature `CgVoicemail` to be enabled.
#[component]
pub fn Voicemail(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M10.0004 15C10.6281 14.1643 11 13.1256 11 12C11 9.23858 8.76142 7 6 7C3.23858 7 1 9.23858 1 12C1 14.7614 3.23858 17 6 17H18C20.7614 17 23 14.7614 23 12C23 9.23858 20.7614 7 18 7C15.2386 7 13 9.23858 13 12C13 13.1256 13.3719 14.1643 13.9996 15H10.0004ZM6 15C7.65685 15 9 13.6569 9 12C9 10.3431 7.65685 9 6 9C4.34315 9 3 10.3431 3 12C3 13.6569 4.34315 15 6 15ZM18 15C19.6569 15 21 13.6569 21 12C21 10.3431 19.6569 9 18 9C16.3431 9 15 10.3431 15 12C15 13.6569 16.3431 15 18 15Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
