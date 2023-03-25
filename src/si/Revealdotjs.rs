#[cfg(feature = "SiRevealdotjs")]
use leptos::*;
#[cfg(feature = "SiRevealdotjs")]
///This icon requires the feature `SiRevealdotjs` to be enabled.
#[component]
pub fn Revealdotjs(
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
        "M4.271 1.352a.774.774 0 0 0-.787.775v19.761c0 .49.45.857.93.758l6.676-1.382-2.77-.614-3.675.762V2.607l3.101.686 2.777-.574-6.097-1.35a.774.774 0 0 0-.155-.017zm15.315.002L5.145 4.344v15.092l14.43 3.195a.774.774 0 0 0 .94-.758V2.111a.773.773 0 0 0-.93-.757zM2.984 4.79l-2.367.49A.774.774 0 0 0 0 6.04v11.639a.774.774 0 0 0 .607.754l2.377.525V4.791zm18.034.252V6.23l1.822.405v11.011l-1.822.377v1.186l2.365-.49A.774.774 0 0 0 24 17.96V6.322a.774.774 0 0 0-.607-.754l-2.375-.525z"
        /> < title > { title } < / title > < / svg >
    }
}
