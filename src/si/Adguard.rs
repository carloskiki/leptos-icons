#[cfg(feature = "SiAdguard")]
use leptos::*;
#[cfg(feature = "SiAdguard")]
///This icon requires the feature `SiAdguard` to be enabled.
#[component]
pub fn Adguard(
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
        "M12 0C8.249 0 3.725.861 0 2.755 0 6.845-.051 17.037 12 24 24.051 17.037 24 6.845 24 2.755 20.275.861 15.751 0 12 0zm-.106 15.429L6.857 9.612c.331-.239 1.75-1.143 2.794.042l2.187 2.588c.009-.001 5.801-5.948 5.815-5.938.246-.22.694-.503 1.204-.101l-6.963 9.226z"
        /> < title > { title } < / title > < / svg >
    }
}
