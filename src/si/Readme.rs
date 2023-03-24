#[cfg(feature = "SiReadme")]
use leptos::*;
#[cfg(feature = "SiReadme")]
///This icon requires the feature `SiReadme` to be enabled.
#[component]
pub fn Readme(
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
        "M22.0113 3.269h-5.8219a4.2894 4.2894 0 0 0-4.1854 3.3452A4.2894 4.2894 0 0 0 7.8186 3.269h-5.818A2.0007 2.0007 0 0 0 0 5.2697v10.2434a2.0007 2.0007 0 0 0 2.0007 2.0007h3.7372c4.2574 0 5.5299 1.0244 6.138 3.133a.112.112 0 0 0 .1121.084h.024a.112.112 0 0 0 .112-.084c.6122-2.1086 1.8847-3.133 6.138-3.133h3.7373A2.0007 2.0007 0 0 0 24 15.5131V5.2697a2.0007 2.0007 0 0 0-1.9887-2.0006Zm-11.928 11.0557a.144.144 0 0 1-.144.144H3.2571a.144.144 0 0 1-.144-.144v-.9523a.144.144 0 0 1 .144-.144h6.6822a.144.144 0 0 1 .144.144zm0-2.5368a.144.144 0 0 1-.144.144H3.2571a.144.144 0 0 1-.144-.144v-.9523a.144.144 0 0 1 .144-.144h6.6822a.144.144 0 0 1 .144.144zm0-2.5368a.144.144 0 0 1-.144.144H3.2571a.144.144 0 0 1-.144-.144v-.9524a.144.144 0 0 1 .144-.144h6.6822a.144.144 0 0 1 .144.144zm10.8037 5.0696a.144.144 0 0 1-.144.144h-6.6823a.144.144 0 0 1-.144-.144v-.9523a.144.144 0 0 1 .144-.144h6.6822a.144.144 0 0 1 .144.144zm0-2.5368a.144.144 0 0 1-.144.144h-6.6823a.144.144 0 0 1-.144-.144v-.9523a.144.144 0 0 1 .144-.144h6.6822a.144.144 0 0 1 .144.144zm0-2.5368a.144.144 0 0 1-.144.144h-6.6823a.144.144 0 0 1-.144-.144v-.9484a.144.144 0 0 1 .144-.144h6.6822a.144.144 0 0 1 .144.144v.9524z"
        /> < title > { title } < / title > < / svg >
    }
}
