#[cfg(feature = "FaRegularBellSlash")]
use leptos::*;
#[cfg(feature = "FaRegularBellSlash")]
///This icon requires the feature `FaRegularBellSlash` to be enabled.
#[component]
pub fn BellSlash(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M38.8 5.1C28.4-3.1 13.3-1.2 5.1 9.2S-1.2 34.7 9.2 42.9l592 464c10.4 8.2 25.5 6.3 33.7-4.1s6.3-25.5-4.1-33.7L542.6 400c2.7-7.8 1.3-16.5-3.9-23l-14.9-18.6C495.5 322.9 480 278.8 480 233.4V200c0-75.8-55.5-138.6-128-150.1V32c0-17.7-14.3-32-32-32s-32 14.3-32 32V49.9c-43.9 7-81.5 32.7-104.4 68.7L38.8 5.1zM221.7 148.4C239.6 117.1 273.3 96 312 96h8 8c57.4 0 104 46.6 104 104v33.4c0 32.7 6.4 64.8 18.7 94.5L221.7 148.4zm-61.7 85c0 45.4-15.5 89.5-43.8 124.9L101.3 377c-5.8 7.2-6.9 17.1-2.9 25.4s12.4 13.6 21.6 13.6H406.2l-60.9-48H168.3c21.2-32.8 34.4-70.3 38.4-109.1L160 222.1v11.4zM384 448H320 256c0 17 6.7 33.3 18.7 45.3s28.3 18.7 45.3 18.7s33.3-6.7 45.3-18.7s18.7-28.3 18.7-45.3z"
        /> < title > { title } < / title > < / svg >
    }
}
