#[cfg(feature = "RiLogosFillSkype")]
use leptos::*;
#[cfg(feature = "RiLogosFillSkype")]
///This icon requires the feature `RiLogosFillSkype` to be enabled.
#[component]
pub fn Skype(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M13.31 20.4a8.5 8.5 0 0 1-9.71-9.71 5.25 5.25 0 0 1 7.09-7.09 8.5 8.5 0 0 1 9.71 9.71 5.25 5.25 0 0 1-7.09 7.09zm-1.258-3.244h-.04c2.872 0 4.303-1.386 4.303-3.243 0-1.198-.551-2.471-2.726-2.958l-1.983-.44c-.755-.172-1.622-.4-1.622-1.115s.62-1.213 1.724-1.213c2.23 0 2.027 1.528 3.131 1.528.576 0 1.093-.342 1.093-.93 0-1.37-2.197-2.4-4.056-2.4-2.021 0-4.173.859-4.173 3.144 0 1.098.394 2.27 2.56 2.813l2.689.671c.816.202 1.018.659 1.018 1.072 0 .687-.684 1.358-1.918 1.358-2.417 0-2.078-1.857-3.374-1.857-.58 0-1.003.398-1.003.971 0 1.114 1.352 2.598 4.377 2.598z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
