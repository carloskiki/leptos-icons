#[cfg(feature = "BiLogosTumblr")]
use leptos::*;
#[cfg(feature = "BiLogosTumblr")]
///This icon requires the feature `BiLogosTumblr` to be enabled.
#[component]
pub fn Tumblr(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M14.078 20.953c-2.692 0-4.699-1.385-4.699-4.7v-5.308H6.931V8.07c2.694-.699 3.821-3.017 3.95-5.023h2.796v4.558h3.263v3.34h-3.263v4.622c0 1.386.699 1.864 1.813 1.864h1.58v3.522h-2.992z"
        /> < title > { title } < / title > < / svg >
    }
}
