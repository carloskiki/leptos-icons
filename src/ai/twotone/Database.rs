#[cfg(feature = "AiTwotoneDatabase")]
use leptos::*;
#[cfg(feature = "AiTwotoneDatabase")]
///This icon requires the feature `AiTwotoneDatabase` to be enabled.
#[component]
pub fn Database(
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
        stroke_witdh = "0" style = style viewBox = "0 0 1024 1024" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill = "#D8D8D8" d =
        "M232 616h560V408H232v208zm112-144c22.1 0 40 17.9 40 40s-17.9 40-40 40-40-17.9-40-40 17.9-40 40-40zM232 888h560V680H232v208zm112-144c22.1 0 40 17.9 40 40s-17.9 40-40 40-40-17.9-40-40 17.9-40 40-40zM232 344h560V136H232v208zm112-144c22.1 0 40 17.9 40 40s-17.9 40-40 40-40-17.9-40-40 17.9-40 40-40z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M304 512a40 40 0 1 0 80 0 40 40 0 1 0-80 0zm0 272a40 40 0 1 0 80 0 40 40 0 1 0-80 0zm0-544a40 40 0 1 0 80 0 40 40 0 1 0-80 0z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M832 64H192c-17.7 0-32 14.3-32 32v832c0 17.7 14.3 32 32 32h640c17.7 0 32-14.3 32-32V96c0-17.7-14.3-32-32-32zm-40 824H232V680h560v208zm0-272H232V408h560v208zm0-272H232V136h560v208z"
        /> < title > { title } < / title > < / svg >
    }
}
