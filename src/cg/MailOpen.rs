#[cfg(feature = "CgMailOpen")]
use leptos::*;
#[cfg(feature = "CgMailOpen")]
///This icon requires the feature `CgMailOpen` to be enabled.
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M3.05002 10.0151C2.79178 9.30708 2.94668 8.48233 3.51474 7.91427L9.8787 1.55031C11.0503 0.378738 12.9498 0.378738 14.1213 1.55031L20.4853 7.91427C21.0534 8.48239 21.2083 9.30727 20.9499 10.0154C20.9824 10.1139 20.9999 10.2191 20.9999 10.3285V21.3285C20.9999 22.4331 20.1045 23.3285 18.9999 23.3285H4.99994C3.89537 23.3285 2.99994 22.4331 2.99994 21.3285V10.3285C2.99994 10.219 3.01752 10.1137 3.05002 10.0151ZM4.92896 9.32848L11.2929 2.96452C11.6834 2.574 12.3166 2.574 12.7071 2.96452L19.0711 9.32848H19.0408V9.3588L12.7071 15.6924C12.3166 16.083 11.6834 16.083 11.2929 15.6924L4.92896 9.32848ZM18.9999 12.2281L14.1213 17.1067C12.9498 18.2782 11.0503 18.2782 9.8787 17.1067L4.99994 12.2279L4.99994 21.3285H18.9999V12.2281Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
