#[cfg(feature = "CgFontHeight")]
use leptos::*;
#[cfg(feature = "CgFontHeight")]
///This icon requires the feature `CgFontHeight` to be enabled.
#[component]
pub fn FontHeight(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 5V3H3V5H21Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 19V21H3V19H21Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule =
        "evenodd" d =
        "M12.0001 7.37636C11.602 7.35207 11.2112 7.56874 11.0325 7.95204L7.65154 15.2025C7.41815 15.7031 7.6347 16.2981 8.13522 16.5315C8.63577 16.7649 9.23074 16.5484 9.46417 16.0477L9.95278 14.9999H14.0473L14.5359 16.0477C14.7693 16.5484 15.3643 16.7649 15.8648 16.5315C16.3654 16.2981 16.5819 15.7031 16.3485 15.2025L12.9676 7.95204C12.7888 7.56874 12.3981 7.35207 12.0001 7.37636ZM13.1147 12.9999H10.8854L12.0001 10.6095L13.1147 12.9999Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
