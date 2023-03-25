#[cfg(feature = "BiSolidBrain")]
use leptos::*;
#[cfg(feature = "BiSolidBrain")]
///This icon requires the feature `BiSolidBrain` to be enabled.
#[component]
pub fn Brain(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M3.299 17.596c.432 1.332 1.745 2.182 3.146 2.182H6.5A2.78 2.78 0 0 0 9.223 22c.457 0 .884-.115 1.262-.313a.992.992 0 0 0 .515-.882V3.027a.997.997 0 0 0-.785-.983 2.324 2.324 0 0 0-1.479.201c-.744.356-1.18 1.151-1.18 1.978v.055a2.778 2.778 0 0 0-2.744 4.433A3.327 3.327 0 0 0 2 12c0 1.178.611 2.211 1.533 2.812-.43.771-.571 1.746-.234 2.784zm15.889-8.885a2.778 2.778 0 0 0-2.744-4.433v-.055c0-.826-.437-1.622-1.181-1.978a2.32 2.32 0 0 0-1.478-.201.998.998 0 0 0-.785.983v17.777c0 .365.192.712.516.882.378.199.804.314 1.261.314a2.78 2.78 0 0 0 2.723-2.223h.056c1.4 0 2.714-.85 3.146-2.182.337-1.038.196-2.013-.234-2.784A3.35 3.35 0 0 0 22 12a3.327 3.327 0 0 0-2.812-3.289z"
        /> < title > { title } < / title > < / svg >
    }
}
