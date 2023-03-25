#[cfg(feature = "CgArrowLongRightC")]
use leptos::*;
#[cfg(feature = "CgArrowLongRightC")]
///This icon requires the feature `CgArrowLongRightC` to be enabled.
#[component]
pub fn ArrowLongRightC(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M18.73 7.75739L22.9798 11.9929L18.7443 16.2426L17.3277 14.8308L19.142 13.0103L6.85364 13.0525C6.44678 14.219 5.33954 15.0584 4.03368 15.0642C2.37684 15.0717 1.02767 13.7346 1.02023 12.0777C1.01279 10.4209 2.34989 9.07173 4.00673 9.06429C5.31328 9.05842 6.4285 9.88867 6.84531 11.0525L19.1607 11.0103L17.3182 9.17398L18.73 7.75739ZM5.02019 12.0598C5.01771 11.5075 4.56799 11.0618 4.01571 11.0643C3.46343 11.0667 3.01773 11.5165 3.02021 12.0687C3.02269 12.621 3.47242 13.0667 4.02469 13.0642C4.57697 13.0618 5.02267 12.612 5.02019 12.0598Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
