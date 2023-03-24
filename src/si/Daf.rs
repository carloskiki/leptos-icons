#[cfg(feature = "SiDaf")]
use leptos::*;
#[cfg(feature = "SiDaf")]
///This icon requires the feature `SiDaf` to be enabled.
#[component]
pub fn Daf(
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
        "M19.649 12.782h-2.643V8.723H24v1.183h-4.351v.723h4.277v1.147h-4.277zm-7.51-3.039l-1.831 3.05H7.76l2.414-4.07h3.924l2.424 4.07h-5.364l.64-1.06h1.534zM.004 12.785V8.741h4.99c1.62 0 2.773.738 2.773 1.994 0 1.196-.914 2.05-2.82 2.05zm4.008-1.034c.621 0 .985-.53.985-.935 0-.413-.325-.896-.967-.896H2.671v1.831zM0 13.731h23.926v1.546H0Z"
        /> < title > { title } < / title > < / svg >
    }
}
