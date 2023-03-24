#[cfg(feature = "FiHardDrive")]
use leptos::*;
#[cfg(feature = "FiHardDrive")]
///This icon requires the feature `FiHardDrive` to be enabled.
#[component]
pub fn HardDrive(
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
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < line xmlns = "http://www.w3.org/2000/svg" x1 =
        "22" y1 = "12" x2 = "2" y2 = "12" />< path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "6" y1 = "16" x2 = "6.01" y2 =
        "16" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "10" y1 = "16" x2 =
        "10.01" y2 = "16" /> < title > { title } < / title > < / svg >
    }
}
