#[cfg(feature = "HiMdSolidFolderOpen")]
use leptos::*;
#[cfg(feature = "HiMdSolidFolderOpen")]
///This icon requires the feature `HiMdSolidFolderOpen` to be enabled.
#[component]
pub fn FolderOpen(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.75 3C3.7835 3 3 3.7835 3 4.75V7.50159C3.03443 7.50053 3.06902 7.5 3.10378 7.5H16.8959C16.9307 7.5 16.9654 7.50054 17 7.50161V6.75C17 5.7835 16.2165 5 15.25 5H11.4142C11.3479 5 11.2843 4.97366 11.2374 4.92678L9.82322 3.51256C9.49504 3.18437 9.04992 3 8.58579 3H4.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.10373 9C1.92632 9 1.08486 10.1393 1.43112 11.2647L2.81573 15.7647C3.04167 16.4989 3.72009 17 4.48835 17H15.5112C16.2795 17 16.9579 16.4989 17.1838 15.7647L18.5684 11.2647C18.9147 10.1393 18.0732 9 16.8958 9H3.10373Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
