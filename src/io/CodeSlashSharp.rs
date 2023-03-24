#[cfg(feature = "IoCodeSlashSharp")]
use leptos::*;
#[cfg(feature = "IoCodeSlashSharp")]
///This icon requires the feature `IoCodeSlashSharp` to be enabled.
#[component]
pub fn CodeSlashSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < polygon xmlns = "http://www.w3.org/2000/svg"
        points =
        "161.98 397.63 0 256 161.98 114.37 189.63 145.98 64 256 189.63 366.02 161.98 397.63"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "350.02 397.63 322.37 366.02 448 256 322.37 145.98 350.02 114.37 512 256 350.02 397.63"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "222.15 442 182 430.08 289.85 70 330 81.92 222.15 442" /> < title > { title } < /
        title > < / svg >
    }
}
