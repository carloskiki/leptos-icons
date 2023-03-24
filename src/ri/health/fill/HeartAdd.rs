#[cfg(feature = "RiHealthFillHeartAdd")]
use leptos::*;
#[cfg(feature = "RiHealthFillHeartAdd")]
///This icon requires the feature `RiHealthFillHeartAdd` to be enabled.
#[component]
pub fn HeartAdd(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M19 14v3h3v2h-3v3h-2v-3h-3v-2h3v-3h2zm1.243-9.243c2.16 2.166 2.329 5.557.507 7.91C19.926 12.24 18.99 12 18 12c-3.314 0-6 2.686-6 6 0 1.009.249 1.96.689 2.794l-.69.691-8.478-8.492c-2.104-2.356-2.025-5.974.236-8.236 2.265-2.264 5.888-2.34 8.244-.228 2.349-2.109 5.979-2.039 8.242.228z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
