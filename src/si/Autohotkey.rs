#[cfg(feature = "SiAutohotkey")]
use leptos::*;
#[cfg(feature = "SiAutohotkey")]
///This icon requires the feature `SiAutohotkey` to be enabled.
#[component]
pub fn Autohotkey(
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
        "M20.514.508V.51H1.803C1.053.51.079 1.135 0 2.27v17.133h.002v2.325c.08 1.136 1.05 1.763 1.8 1.763h1.505l.002-.002h18.869c1.256-.053 1.766-1.066 1.822-1.699v-3.023h-.002V2.209c-.056-.633-.567-1.648-1.824-1.701h-1.66zM3.412 1.623h17.154c.898 0 1.618.72 1.618 1.617v16.64c0 .898-.72 1.62-1.618 1.62H3.412a1.616 1.616 0 01-1.619-1.62V3.24c0-.897.722-1.617 1.62-1.617zm3.315 12.412l-1.895 5.037h.703l.526-1.467h2.02l.497 1.467h.744l-1.824-5.037h-.771zm8.43.008v5.037h.679v-1.767l.793-.758 1.76 2.525h.884l-2.154-3.002 2.098-2.035h-.94l-2.441 2.441v-2.441h-.68zm-5.153.027v5.037h.682v-2.351h2.628v2.351h.682V14.07h-.682v2.084h-2.628V14.07h-.682zm-2.926.717h.014l.742 2.217H6.271l.807-2.217z"
        /> < title > { title } < / title > < / svg >
    }
}
