#[cfg(feature = "SiSemanticscholar")]
use leptos::*;
#[cfg(feature = "SiSemanticscholar")]
///This icon requires the feature `SiSemanticscholar` to be enabled.
#[component]
pub fn Semanticscholar(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M24 8.609c-.848.536-1.436.83-2.146 1.245-4.152 2.509-8.15 5.295-11.247 8.981l-1.488 1.817-4.568-7.268c1.021.814 3.564 3.098 4.603 3.599l3.356-2.526c2.336-1.644 8.946-5.226 11.49-5.848ZM8.046 15.201c.346.277.692.537.969.744.761-3.668.121-7.613-1.886-11.039 3.374-.052 6.731-.087 10.105-.139a14.794 14.794 0 0 1 1.298 5.295c.294-.156.588-.294.883-.433-.104-1.868-.641-3.91-1.662-6.263-4.602-.018-9.188-.018-13.79-.018 2.993 3.547 4.36 7.839 4.083 11.853Zm-.623-.484c.087.086.191.155.277.225-.138-3.409-1.419-6.887-3.824-9.881H1.73c3.098 2.855 4.984 6.299 5.693 9.656Zm-.744-.658c.104.087.208.173.329.277-.9-2.526-2.492-5.018-4.741-7.198H0c2.89 2.076 5.122 4.481 6.679 6.921Z"
        /> < title > { title } < / title > < / svg >
    }
}
