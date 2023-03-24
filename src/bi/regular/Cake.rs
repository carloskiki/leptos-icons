#[cfg(feature = "BiRegularCake")]
use leptos::*;
#[cfg(feature = "BiRegularCake")]
///This icon requires the feature `BiRegularCake` to be enabled.
#[component]
pub fn Cake(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19 5h-6V2h-2v3H5C3.346 5 2 6.346 2 8v10c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V8c0-1.654-1.346-3-3-3zM5 7h14a1 1 0 0 1 1 1l.001 3.12c-.896.228-1.469.734-1.916 1.132-.507.45-.842.748-1.588.748-.745 0-1.08-.298-1.587-.747-.595-.529-1.409-1.253-2.915-1.253-1.505 0-2.319.724-2.914 1.253-.507.45-.841.747-1.586.747-.743 0-1.077-.297-1.582-.747-.447-.398-1.018-.905-1.913-1.133V8a1 1 0 0 1 1-1zM4 18v-4.714c.191.123.374.274.583.461C5.178 14.276 5.991 15 7.495 15c1.505 0 2.319-.724 2.914-1.253.507-.45.841-.747 1.586-.747s1.08.298 1.587.747c.595.529 1.409 1.253 2.915 1.253s2.321-.724 2.916-1.253c.211-.188.395-.34.588-.464L20.002 18H4z"
        /> < title > { title } < / title > < / svg >
    }
}
