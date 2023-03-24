#[cfg(feature = "SiPenpot")]
use leptos::*;
#[cfg(feature = "SiPenpot")]
///This icon requires the feature `SiPenpot` to be enabled.
#[component]
pub fn Penpot(
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
        "M7.654 0 5.13 3.554v2.01L2.934 6.608l-.02-.009v13.109l8.563 4.045L12 24l.523-.247 8.563-4.045V6.6l-.017.008-2.196-1.045V3.555l-.077-.108L16.349.001l-2.524 3.554v.004L11.989.973l-1.823 2.566-.065-.091zm.447 2.065.976 1.374H6.232l.964-1.358zm8.694 0 .976 1.374h-2.845l.965-1.358zm-4.36.971.976 1.375h-2.845l.965-1.359zM5.962 4.132h1.35v4.544l-1.35-.638Zm2.042 0h1.343v5.506l-1.343-.635zm6.652 0h1.35V9l-1.35.637V4.132zm2.042 0h1.343v3.905l-1.343.634zm-6.402.972h1.35v5.62l-1.35-.638zm2.042 0h1.343v4.993l-1.343.634zm6.534 1.493 1.188.486-1.188.561zM5.13 6.6v1.047l-1.187-.561ZM3.96 8.251l7.517 3.55v10.795l-7.516-3.55zm16.08 0v10.794l-7.517 3.55V11.802z"
        /> < title > { title } < / title > < / svg >
    }
}
