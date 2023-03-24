#[cfg(feature = "RiLogosFillLine")]
use leptos::*;
#[cfg(feature = "RiLogosFillLine")]
///This icon requires the feature `RiLogosFillLine` to be enabled.
#[component]
pub fn Line(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M18.663 10.84a.526.526 0 0 1-.526.525h-1.462v.938h1.462a.525.525 0 1 1 0 1.049H16.15a.526.526 0 0 1-.522-.524V8.852c0-.287.235-.525.525-.525h1.988a.525.525 0 0 1-.003 1.05h-1.462v.938h1.462c.291 0 .526.237.526.525zm-4.098 2.485a.538.538 0 0 1-.166.025.515.515 0 0 1-.425-.208l-2.036-2.764v2.45a.525.525 0 0 1-1.047 0V8.852a.522.522 0 0 1 .52-.523c.162 0 .312.086.412.211l2.052 2.775V8.852c0-.287.235-.525.525-.525.287 0 .525.238.525.525v3.976a.524.524 0 0 1-.36.497zm-4.95.027a.526.526 0 0 1-.523-.524V8.852c0-.287.236-.525.525-.525.289 0 .524.238.524.525v3.976a.527.527 0 0 1-.526.524zm-1.53 0H6.098a.528.528 0 0 1-.525-.524V8.852a.527.527 0 0 1 1.05 0v3.45h1.464a.525.525 0 0 1 0 1.05zM12 2.572c-5.513 0-10 3.643-10 8.118 0 4.01 3.558 7.369 8.363 8.007.325.068.769.215.881.492.1.25.066.638.032.9l-.137.85c-.037.25-.2.988.874.537 1.076-.449 5.764-3.398 7.864-5.812C21.313 14.089 22 12.477 22 10.69c0-4.475-4.488-8.118-10-8.118z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
