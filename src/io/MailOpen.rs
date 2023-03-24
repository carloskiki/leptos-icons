#[cfg(feature = "IoMailOpen")]
use leptos::*;
#[cfg(feature = "IoMailOpen")]
///This icon requires the feature `IoMailOpen` to be enabled.
#[component]
pub fn MailOpen(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M448.67,154.45,274.1,68.2a41.1,41.1,0,0,0-36.2,0L63.33,154.45A55.6,55.6,0,0,0,32,204.53V389.14c0,30.88,25.42,56,56.67,56H423.33c31.25,0,56.67-25.12,56.67-56V204.53A55.6,55.6,0,0,0,448.67,154.45ZM252.38,96.82a8.22,8.22,0,0,1,7.24,0L429,180.48l-172,85a8.22,8.22,0,0,1-7.24,0L80.35,181.81Z"
        /> < title > { title } < / title > < / svg >
    }
}
