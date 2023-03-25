#[cfg(feature = "RiEditorRoundedCorner")]
use leptos::*;
#[cfg(feature = "RiEditorRoundedCorner")]
///This icon requires the feature `RiEditorRoundedCorner` to be enabled.
#[component]
pub fn RoundedCorner(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M21 19v2h-2v-2h2zm-4 0v2h-2v-2h2zm-4 0v2h-2v-2h2zm-4 0v2H7v-2h2zm-4 0v2H3v-2h2zm16-4v2h-2v-2h2zM5 15v2H3v-2h2zm0-4v2H3v-2h2zm11-8c2.687 0 4.882 2.124 4.995 4.783L21 8v5h-2V8c0-1.591-1.255-2.903-2.824-2.995L16 5h-5V3h5zM5 7v2H3V7h2zm0-4v2H3V3h2zm4 0v2H7V3h2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
