#[cfg(feature = "SiLinkfire")]
use leptos::*;
#[cfg(feature = "SiLinkfire")]
///This icon requires the feature `SiLinkfire` to be enabled.
#[component]
pub fn Linkfire(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M10.08 1.997A10.041 10.042 0 0 0 0 12.006a10.041 10.042 0 0 0 10.08 10 10.037 10.038 0 0 0 7.12-2.933c2.883-2.796 4.509-8.065 5.775-12.861l.038-.152.09-.343a.227.227 0 0 0-.038-.197.234.234 0 0 0-.185-.09h-1.711a.23.23 0 0 1-.215-.14.227.227 0 0 1 .053-.25l2.925-2.646v-.011a.23.23 0 0 0 .053-.246.23.23 0 0 0-.216-.143zm.385 4.645h3.847a1.686 1.686 0 0 1 1.682 1.686v3.844a1.686 1.686 0 0 1-1.682 1.686H10.47a1.686 1.686 0 0 1-1.684-1.686v-.722h1.442v.722a.246.246 0 0 0 .242.242h3.848a.242.242 0 0 0 .242-.242V8.324a.242.242 0 0 0-.242-.242H10.47a.242.242 0 0 0-.242.242v.722H8.784v-.718a1.686 1.686 0 0 1 1.684-1.686zM7.582 9.526h3.84a1.686 1.686 0 0 1 1.684 1.686v.718h-1.44v-.718a.242.242 0 0 0-.24-.242H7.577a.242.242 0 0 0-.242.242v3.847a.242.242 0 0 0 .242.242h3.847a.242.242 0 0 0 .241-.242v-.718h1.448v.718a1.686 1.686 0 0 1-1.686 1.682H7.582a1.686 1.686 0 0 1-1.69-1.682v-3.847a1.686 1.686 0 0 1 1.69-1.686z"
        /> < title > { title } < / title > < / svg >
    }
}
