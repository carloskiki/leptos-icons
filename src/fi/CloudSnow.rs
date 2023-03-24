#[cfg(feature = "FiCloudSnow")]
use leptos::*;
#[cfg(feature = "FiCloudSnow")]
///This icon requires the feature `FiCloudSnow` to be enabled.
#[component]
pub fn CloudSnow(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 17.58A5 5 0 0 0 18 8h-1.26A8 8 0 1 0 4 16.25" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "8" y1 = "16" x2 = "8.01" y2 = "16" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "8" y1 = "20" x2 = "8.01" y2 = "20" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "12" y1 = "18" x2 = "12.01" y2 =
        "18" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "12" y1 = "22" x2 =
        "12.01" y2 = "22" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "16" y1 =
        "16" x2 = "16.01" y2 = "16" />< line xmlns = "http://www.w3.org/2000/svg" x1 =
        "16" y1 = "20" x2 = "16.01" y2 = "20" /> < title > { title } < / title > < / svg
        >
    }
}
