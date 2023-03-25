#[cfg(feature = "SiHaskell")]
use leptos::*;
#[cfg(feature = "SiHaskell")]
///This icon requires the feature `SiHaskell` to be enabled.
#[component]
pub fn Haskell(
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
        "M0 3.535L5.647 12 0 20.465h4.235L9.883 12 4.235 3.535zm5.647 0L11.294 12l-5.647 8.465h4.235l3.53-5.29 3.53 5.29h4.234L9.883 3.535zm8.941 4.938l1.883 2.822H24V8.473zm2.824 4.232l1.882 2.822H24v-2.822z"
        /> < title > { title } < / title > < / svg >
    }
}
