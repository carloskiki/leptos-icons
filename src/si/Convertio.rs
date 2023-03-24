#[cfg(feature = "SiConvertio")]
use leptos::*;
#[cfg(feature = "SiConvertio")]
///This icon requires the feature `SiConvertio` to be enabled.
#[component]
pub fn Convertio(
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
        "M12 .037C5.373.037 0 5.394 0 12c0 6.606 5.373 11.963 12 11.963 6.628 0 12-5.357 12-11.963C24 5.394 18.627.037 12 .037zm-.541 4.8c1.91-.13 3.876.395 5.432 1.934 1.426 1.437 2.51 3.44 2.488 5.317h2.133l-4.444 4.963-4.445-4.963h2.313c-.001-1.724-.427-2.742-1.78-4.076-1.325-1.336-2.667-2.11-4.978-2.303a9.245 9.245 0 013.281-.871zM6.934 6.95l4.445 4.963H9.066c0 1.724.426 2.742 1.778 4.076 1.326 1.336 2.667 2.112 4.978 2.305-2.684 1.268-6.22 1.398-8.71-1.064-1.427-1.437-2.512-3.44-2.489-5.317H2.488L6.934 6.95Z"
        /> < title > { title } < / title > < / svg >
    }
}
