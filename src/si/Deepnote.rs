#[cfg(feature = "SiDeepnote")]
use leptos::*;
#[cfg(feature = "SiDeepnote")]
///This icon requires the feature `SiDeepnote` to be enabled.
#[component]
pub fn Deepnote(
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
        "M10.22 11.506l.111.018c-.038-.006-.075-.011-.111-.018zm4.475 8.073c.636-2.816-2.172-4.8-6.955-4.814L.713 24h9.586c.132-.025.256-.056.384-.085 2.258-1.057 3.598-2.501 4.012-4.336zM10.299 24h.203l.021-.01c-.075.003-.148.008-.224.01zM24 11.319C24 3.15 18.711-.597 8.134.077L0 11.319h7.568c3.323 0 8.457.719 8.457 6.153 0 3.622-1.909 5.798-5.727 6.528.099-.003.194-.009.291-.013l-.011.001-.076.012h.912l.247-.077C19.885 23.27 24 19.07 24 11.319z"
        /> < title > { title } < / title > < / svg >
    }
}
