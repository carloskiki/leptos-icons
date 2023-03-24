#[cfg(feature = "FiShare2")]
use leptos::*;
#[cfg(feature = "FiShare2")]
///This icon requires the feature `FiShare2` to be enabled.
#[component]
pub fn Share2(
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
        "18" cy = "5" r = "3" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "6" cy
        = "12" r = "3" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "18" cy =
        "19" r = "3" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "8.59" y1 =
        "13.51" x2 = "15.42" y2 = "17.49" />< line xmlns = "http://www.w3.org/2000/svg"
        x1 = "15.41" y1 = "6.51" x2 = "8.59" y2 = "10.49" /> < title > { title } < /
        title > < / svg >
    }
}
