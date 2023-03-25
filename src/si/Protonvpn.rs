#[cfg(feature = "SiProtonvpn")]
use leptos::*;
#[cfg(feature = "SiProtonvpn")]
///This icon requires the feature `SiProtonvpn` to be enabled.
#[component]
pub fn Protonvpn(
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
        "M22.971 2.68a2.68 2.68 0 1 0-5.361 0v.082L5.927 6.534a2.68 2.68 0 1 0-2.218 4.187c.279-.004.555-.052.819-.142l7.673 9.69a2.68 2.68 0 1 0 4.211-.984l4.08-13.937a2.669 2.669 0 0 0 2.479-2.668zm-8.29 15.905c-.414.005-.82.107-1.187.296L5.998 9.393a2.66 2.66 0 0 0 .38-1.115L18.31 4.47c.124.14.263.267.415.379l-4.033 13.735h-.012z"
        /> < title > { title } < / title > < / svg >
    }
}
