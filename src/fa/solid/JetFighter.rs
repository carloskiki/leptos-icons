#[cfg(feature = "FaSolidJetFighter")]
use leptos::*;
#[cfg(feature = "FaSolidJetFighter")]
///This icon requires the feature `FaSolidJetFighter` to be enabled.
#[component]
pub fn JetFighter(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M160 24c0-13.3 10.7-24 24-24H296c13.3 0 24 10.7 24 24s-10.7 24-24 24H280L384 192H500.4c7.7 0 15.3 1.4 22.5 4.1L625 234.4c9 3.4 15 12 15 21.6s-6 18.2-15 21.6L522.9 315.9c-7.2 2.7-14.8 4.1-22.5 4.1H384L280 464h16c13.3 0 24 10.7 24 24s-10.7 24-24 24H184c-13.3 0-24-10.7-24-24s10.7-24 24-24h8V320H160l-54.6 54.6c-6 6-14.1 9.4-22.6 9.4H64c-17.7 0-32-14.3-32-32V288c-17.7 0-32-14.3-32-32s14.3-32 32-32V160c0-17.7 14.3-32 32-32H82.7c8.5 0 16.6 3.4 22.6 9.4L160 192h32V48h-8c-13.3 0-24-10.7-24-24zM80 240c-8.8 0-16 7.2-16 16s7.2 16 16 16h64c8.8 0 16-7.2 16-16s-7.2-16-16-16H80z"
        /> < title > { title } < / title > < / svg >
    }
}
