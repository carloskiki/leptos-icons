#[cfg(feature = "SiCitrix")]
use leptos::*;
#[cfg(feature = "SiCitrix")]
///This icon requires the feature `SiCitrix` to be enabled.
#[component]
pub fn Citrix(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M11.983 0a1.78 1.78 0 0 0-1.78 1.78 1.78 1.78 0 0 0 1.78 1.78 1.78 1.78 0 0 0 1.78-1.78A1.78 1.78 0 0 0 11.983 0zM5.17 5.991a1.026 1.026 0 0 0-1.095 1.027c0 .308.136.616.376.822l6.162 7.086-6.401 7.258a1.084 1.084 0 0 0-.309.787c0 .582.48 1.027 1.062 1.027.342 0 .684-.17.89-.444l6.128-7.19 6.162 7.19c.205.274.547.444.89.444.582.035 1.062-.445 1.062-1.027a1.14 1.14 0 0 0-.309-.787l-6.402-7.258 6.162-7.086c.24-.206.377-.514.377-.822v-.034c0-.582-.513-1.027-1.095-.993-.343 0-.65.171-.856.445l-5.957 7.018L6.06 6.436a1.07 1.07 0 0 0-.855-.445z"
        /> < title > { title } < / title > < / svg >
    }
}
