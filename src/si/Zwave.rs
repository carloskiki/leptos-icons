#[cfg(feature = "SiZwave")]
use leptos::*;
#[cfg(feature = "SiZwave")]
///This icon requires the feature `SiZwave` to be enabled.
#[component]
pub fn Zwave(
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
        "M17.156.014C7.69.014 0 7.704 0 17.17h2.3C2.33 8.956 8.973 2.316 17.157 2.316V.014zm0 4.574c-6.932 0-12.584 5.65-12.584 12.582h2.301c.03-5.68 4.633-10.281 10.283-10.281V4.588zm0 5.709a6.837 6.837 0 00-6.845 6.844 6.839 6.839 0 006.845 6.845A6.837 6.837 0 0024 17.141a6.835 6.835 0 00-6.844-6.844zm-2.273 3.174h5.738l-3.058 4.863h3.058l-1.398 2.156h-5.795l3.144-4.922h-3l1.31-2.097Z"
        /> < title > { title } < / title > < / svg >
    }
}
