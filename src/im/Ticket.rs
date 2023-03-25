#[cfg(feature = "ImTicket")]
use leptos::*;
#[cfg(feature = "ImTicket")]
///This icon requires the feature `ImTicket` to be enabled.
#[component]
pub fn Ticket(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M9 5l2 2-4 4-2-2zM15.649 4.649l-1.149-1.149-0.5 0.5c-0.256 0.256-0.61 0.414-1 0.414-0.781 0-1.414-0.633-1.414-1.414 0-0.391 0.158-0.744 0.415-1l0.5-0.5-1.149-1.149c-0.468-0.468-1.235-0.468-1.703 0l-9.297 9.297c-0.468 0.468-0.468 1.235 0 1.703l1.149 1.149 0.499-0.499c0.256-0.256 0.61-0.415 1.001-0.415 0.781 0 1.414 0.633 1.414 1.414 0 0.391-0.158 0.744-0.415 1l-0.5 0.5 1.149 1.149c0.468 0.468 1.234 0.468 1.703 0l9.297-9.297c0.468-0.468 0.468-1.235 0-1.703zM7 13l-4-4 6-6 4 4-6 6z"
        /> < title > { title } < / title > < / svg >
    }
}
