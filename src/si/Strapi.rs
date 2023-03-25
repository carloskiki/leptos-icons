#[cfg(feature = "SiStrapi")]
use leptos::*;
#[cfg(feature = "SiStrapi")]
///This icon requires the feature `SiStrapi` to be enabled.
#[component]
pub fn Strapi(
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
        "M7.684 0v8.035h7.775c.28 0 .502.236.502.483v7.802h8.025V.502A.502.502 0 0023.484 0zm-.5.5L.086 7.607a.251.251 0 00.178.428h6.92zm.5 8.035v7.283c0 .278.224.502.502.502h7.275V9.018c0-.278-.224-.482-.502-.483zm8.277 8.285v6.928c0 .224.271.336.43.178l7.095-7.106z"
        /> < title > { title } < / title > < / svg >
    }
}
