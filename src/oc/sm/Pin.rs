#[cfg(feature = "OcSmPin")]
use leptos::*;
#[cfg(feature = "OcSmPin")]
///This icon requires the feature `OcSmPin` to be enabled.
#[component]
pub fn Pin(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M4.456.734a1.75 1.75 0 0 1 2.826.504l.613 1.327a3.08 3.08 0 0 0 2.084 1.707l2.454.584c1.332.317 1.8 1.972.832 2.94L11.06 10l3.72 3.72a.748.748 0 0 1-.332 1.265.75.75 0 0 1-.729-.205L10 11.06l-2.204 2.205c-.968.968-2.623.5-2.94-.832l-.584-2.454a3.08 3.08 0 0 0-1.707-2.084l-1.327-.613a1.75 1.75 0 0 1-.504-2.826ZM5.92 1.866a.253.253 0 0 0-.183-.142.251.251 0 0 0-.221.07L1.794 5.516a.251.251 0 0 0-.07.221c.015.08.068.149.142.183l1.328.613A4.582 4.582 0 0 1 5.73 9.63l.584 2.454a.251.251 0 0 0 .42.12l5.47-5.47a.25.25 0 0 0-.12-.42L9.63 5.73a4.583 4.583 0 0 1-3.098-2.537Z"
        /> < title > { title } < / title > < / svg >
    }
}
