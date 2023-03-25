#[cfg(feature = "SiHellyhansen")]
use leptos::*;
#[cfg(feature = "SiHellyhansen")]
///This icon requires the feature `SiHellyhansen` to be enabled.
#[component]
pub fn Hellyhansen(
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
        "M22.912 5.945a1.089 1.089 0 10-.002 2.178 1.089 1.089 0 00.002-2.178zm.012.242a.85.85 0 110 1.7.85.85 0 010-1.7zm-.332.375v.952h.18v-.352h.171l.184.352h.207l-.213-.385c.046-.017.19-.067.19-.28 0-.166-.12-.287-.323-.287h-.396zm.18.157h.167c.124 0 .184.057.184.144 0 .089-.065.143-.156.143h-.196v-.287zM0 7.039v11.016h3.684v-3.78h3.523v3.78h1.42l2.15-11.016H7.221v3.854H3.695V7.039H0zm12.127 0L9.988 18.055h3.545V14.2h3.524v3.854h3.697V7.039H17.07v3.78h-3.525v-3.78h-1.418Z"
        /> < title > { title } < / title > < / svg >
    }
}
