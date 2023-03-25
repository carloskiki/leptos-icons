#[cfg(feature = "SiMicrobit")]
use leptos::*;
#[cfg(feature = "SiMicrobit")]
///This icon requires the feature `SiMicrobit` to be enabled.
#[component]
pub fn Microbit(
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
        "M6.857 5.143A6.865 6.865 0 000 12a6.864 6.864 0 006.857 6.857h10.287A6.863 6.863 0 0024 12c0-3.781-3.075-6.857-6.856-6.857zm0 2.744h10.287A4.117 4.117 0 0121.257 12a4.119 4.119 0 01-4.113 4.116H6.857A4.12 4.12 0 012.743 12a4.118 4.118 0 014.114-4.113zm10.168 2.729a1.385 1.385 0 10.003 2.77 1.385 1.385 0 00-.003-2.77zm-10.166 0a1.385 1.385 0 10-.003 2.771 1.385 1.385 0 00.003-2.77Z"
        /> < title > { title } < / title > < / svg >
    }
}
