#[cfg(feature = "IoDocuments")]
use leptos::*;
#[cfg(feature = "IoDocuments")]
///This icon requires the feature `IoDocuments` to be enabled.
#[component]
pub fn Documents(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M298.39,248a4,4,0,0,0,2.86-6.8l-78.4-79.72a4,4,0,0,0-6.85,2.81V236a12,12,0,0,0,12,12Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M197,267A43.67,43.67,0,0,1,184,236V144H112a64.19,64.19,0,0,0-64,64V432a64,64,0,0,0,64,64H256a64,64,0,0,0,64-64V280H228A43.61,43.61,0,0,1,197,267Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M372,120h70.39a4,4,0,0,0,2.86-6.8l-78.4-79.72A4,4,0,0,0,360,36.29V108A12,12,0,0,0,372,120Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M372,152a44.34,44.34,0,0,1-44-44V16H220a60.07,60.07,0,0,0-60,60v36h42.12A40.81,40.81,0,0,1,231,124.14l109.16,111a41.11,41.11,0,0,1,11.83,29V400h53.05c32.51,0,58.95-26.92,58.95-60V152Z"
        /> < title > { title } < / title > < / svg >
    }
}
