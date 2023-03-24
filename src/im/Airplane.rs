#[cfg(feature = "ImAirplane")]
use leptos::*;
#[cfg(feature = "ImAirplane")]
///This icon requires the feature `ImAirplane` to be enabled.
#[component]
pub fn Airplane(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M12 9.999l-2.857-2.857 6.857-5.143-2-2-8.571 3.429-2.698-2.699c-0.778-0.778-1.864-0.964-2.414-0.414s-0.364 1.636 0.414 2.414l2.698 2.698-3.429 8.572 2 2 5.144-6.857 2.857 2.857v4h2l1-3 3-1v-2l-4 0z"
        /> < title > { title } < / title > < / svg >
    }
}
