#[cfg(feature = "IoSearchCircle")]
use leptos::*;
#[cfg(feature = "IoSearchCircle")]
///This icon requires the feature `IoSearchCircle` to be enabled.
#[component]
pub fn SearchCircle(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M256,64C150.13,64,64,150.13,64,256s86.13,192,192,192,192-86.13,192-192S361.87,64,256,64Zm91.31,283.31a16,16,0,0,1-22.62,0l-42.84-42.83a88.08,88.08,0,1,1,22.63-22.63l42.83,42.84A16,16,0,0,1,347.31,347.31Z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "232" cy = "232" r = "56" />
        < title > { title } < / title > < / svg >
    }
}
