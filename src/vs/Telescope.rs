#[cfg(feature = "VsTelescope")]
use leptos::*;
#[cfg(feature = "VsTelescope")]
///This icon requires the feature `VsTelescope` to be enabled.
#[component]
pub fn Telescope(
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
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M11.24 1l.59.24 2.11 4.93-.23.59-3.29 1.41-.59-.24-.17-.41L6.1 9l-.58-.19-.16-.38L2.8 9.49l-.58-.24-.72-1.67.28-.59 2.5-1.06-.18-.41.24-.58L7.9 3.41 7.72 3 8 2.42 11.24 1zM2.5 7.64l.35.85 2.22-.91-.37-.85-2.2.91zm2.74-2.12l1.11 2.45 3-1.28-1.11-2.44-3 1.27zM8.79 3l1.86 4.11 2.29-1.01L11.18 2 8.72 3h.07zM8.5 9.1l3.02 4.9h-1.17l-1.88-3.03v4h-1V9.82L5.58 14h-1.1l1.7-3.9 2.32-1z"
        /> < title > { title } < / title > < / svg >
    }
}
