#[cfg(feature = "FiAperture")]
use leptos::*;
#[cfg(feature = "FiAperture")]
///This icon requires the feature `FiAperture` to be enabled.
#[component]
pub fn Aperture(
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
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "12" cy = "12" r = "10" />< line xmlns = "http://www.w3.org/2000/svg" x1 =
        "14.31" y1 = "8" x2 = "20.05" y2 = "17.94" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "9.69" y1 = "8" x2 = "21.17" y2 = "8" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "7.38" y1 = "12" x2 = "13.12" y2 =
        "2.06" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "9.69" y1 = "16" x2 =
        "3.95" y2 = "6.06" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "14.31" y1
        = "16" x2 = "2.83" y2 = "16" />< line xmlns = "http://www.w3.org/2000/svg" x1 =
        "16.62" y1 = "12" x2 = "10.88" y2 = "21.94" /> < title > { title } < / title > <
        / svg >
    }
}
