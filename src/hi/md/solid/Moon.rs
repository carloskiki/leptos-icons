#[cfg(feature = "HiMdSolidMoon")]
use leptos::*;
#[cfg(feature = "HiMdSolidMoon")]
///This icon requires the feature `HiMdSolidMoon` to be enabled.
#[component]
pub fn Moon(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M7.45519 2.00383C7.68518 2.18752 7.78646 2.48878 7.71414 2.77411C7.57443 3.32534 7.5 3.90336 7.5 4.49984C7.5 8.36583 10.634 11.4998 14.5 11.4998C15.6435 11.4998 16.721 11.2262 17.6724 10.7415C17.9347 10.6079 18.2509 10.6401 18.4809 10.8238C18.7109 11.0075 18.8122 11.3088 18.7399 11.5941C17.8069 15.2754 14.4725 17.9998 10.5 17.9998C5.80558 17.9998 2 14.1943 2 9.49984C2 6.19127 3.89048 3.32555 6.64671 1.92156C6.909 1.78795 7.22519 1.82013 7.45519 2.00383Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
