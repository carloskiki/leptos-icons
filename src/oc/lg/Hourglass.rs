#[cfg(feature = "OcLgHourglass")]
use leptos::*;
#[cfg(feature = "OcLgHourglass")]
///This icon requires the feature `OcLgHourglass` to be enabled.
#[component]
pub fn Hourglass(
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
        "M4.75 2h14.5a.75.75 0 0 1 0 1.5h-.75v2.982a4.75 4.75 0 0 1-2.215 4.017l-2.044 1.29a.25.25 0 0 0 0 .422l2.044 1.29a4.75 4.75 0 0 1 2.215 4.017V20.5h.75a.75.75 0 0 1 0 1.5H4.75a.75.75 0 0 1 0-1.5h.75v-2.982a4.75 4.75 0 0 1 2.215-4.017l2.044-1.29a.25.25 0 0 0 0-.422l-2.044-1.29A4.75 4.75 0 0 1 5.5 6.482V3.5h-.75a.75.75 0 0 1 0-1.5ZM17 3.5H7v2.982A3.25 3.25 0 0 0 8.516 9.23l2.044 1.29a1.75 1.75 0 0 1 0 2.96l-2.044 1.29A3.25 3.25 0 0 0 7 17.518V20.5h10v-2.982a3.25 3.25 0 0 0-1.516-2.748l-2.044-1.29a1.75 1.75 0 0 1 0-2.96l2.044-1.29A3.25 3.25 0 0 0 17 6.482Z"
        /> < title > { title } < / title > < / svg >
    }
}
