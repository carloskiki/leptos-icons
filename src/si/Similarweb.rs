#[cfg(feature = "SiSimilarweb")]
use leptos::*;
#[cfg(feature = "SiSimilarweb")]
///This icon requires the feature `SiSimilarweb` to be enabled.
#[component]
pub fn Similarweb(
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
        "M22.099 5.781c-1.283-2-3.14-3.67-5.27-4.52l-.63-.213a7.433 7.433 0 0 0-2.15-.331c-2.307.01-4.175 1.92-4.175 4.275a4.3 4.3 0 0 0 .867 2.602l-.26-.342c.124.186.26.37.417.556.663.802 1.604 1.635 2.822 2.58 2.999 2.32 4.943 4.378 5.104 6.93.038.344.062.696.062 1.051 0 1.297-.283 2.67-.764 3.635h.005s-.207.377-.077.487c.066.057.21.1.46-.053a12.104 12.104 0 0 0 3.4-3.33 12.111 12.111 0 0 0 2.088-6.635 12.098 12.098 0 0 0-1.9-6.692zm-9.096 8.718-1.878-1.55c-3.934-2.87-5.98-5.966-4.859-9.783a8.73 8.73 0 0 1 .37-1.016v-.004s.278-.583-.327-.295a12.067 12.067 0 0 0-6.292 9.975 12.11 12.11 0 0 0 2.053 7.421 9.394 9.394 0 0 0 2.154 2.168H4.22c4.148 3.053 7.706 1.446 7.706 1.446h.003a4.847 4.847 0 0 0 2.962-4.492 4.855 4.855 0 0 0-1.889-3.87z"
        /> < title > { title } < / title > < / svg >
    }
}
