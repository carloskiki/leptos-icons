#[cfg(feature = "RiLogosLineDisqus")]
use leptos::*;
#[cfg(feature = "RiLogosLineDisqus")]
///This icon requires the feature `RiLogosLineDisqus` to be enabled.
#[component]
pub fn Disqus(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M11.95 2c5.523 0 10 4.477 10 10s-4.477 10-10 10c-2.306 0-4.492-.784-6.249-2.192l-4.718.59 1.72-4.586C2.207 14.614 1.95 13.324 1.95 12c0-5.523 4.477-10 10-10zm0 2c-4.418 0-8 3.582-8 8 0 1.178.254 2.318.738 3.362l.176.38-.847 2.26 2.315-.289.338.297C8.12 19.286 9.978 20 11.95 20c4.418 0 8-3.582 8-8s-3.582-8-8-8zM8 7h3.79c3.42 0 5.44 1.956 5.54 4.729l.003.215v.027c0 2.814-1.962 4.922-5.337 5.025l-.263.004H8V7h3.79H8zm3.831 2.458h-1.108v5.085h1.108c1.566 0 2.625-.845 2.704-2.345l.005-.183v-.028c0-1.6-1.08-2.53-2.709-2.53z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
