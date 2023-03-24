#[cfg(feature = "IoWalletSharp")]
use leptos::*;
#[cfg(feature = "IoWalletSharp")]
///This icon requires the feature `IoWalletSharp` to be enabled.
#[component]
pub fn WalletSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M47.5,104H432V51.52a16,16,0,0,0-19.14-15.69l-368,60.48a16,16,0,0,0-12,10.47A39.69,39.69,0,0,1,47.5,104Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M463.5,128H47.5a16,16,0,0,0-16,16V432a16,16,0,0,0,16,16h416a16,16,0,0,0,16-16V144A16,16,0,0,0,463.5,128ZM368,320a32,32,0,1,1,32-32A32,32,0,0,1,368,320Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M31.33,259.5V116c0-12.33,5.72-18.48,15.42-20,35.2-5.53,108.58-8.5,108.58-8.5s-8.33,16-27.33,16V128c18.5,0,31.33,23.5,31.33,23.5L84.83,236Z"
        /> < title > { title } < / title > < / svg >
    }
}
