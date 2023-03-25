#[cfg(feature = "BiRegularWinkTongue")]
use leptos::*;
#[cfg(feature = "BiRegularWinkTongue")]
///This icon requires the feature `BiRegularWinkTongue` to be enabled.
#[component]
pub fn WinkTongue(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M15.5 9c-2 0-2.5 2-2.5 2h5s-.501-2-2.5-2z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm-2 16v-3h4v3c0 1.103-.897 2-2 2s-2-.897-2-2zm5.856 1.005c.085-.323.144-.656.144-1.005v-1.499C17.589 15.028 18 13 18 13H6s.412 2.028 2 3.501V18c0 .349.059.682.144 1.005A8.005 8.005 0 0 1 4 12c0-4.411 3.589-8 8-8s8 3.589 8 8a8.005 8.005 0 0 1-4.144 7.005z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "8.5" cy = "9.5" r = "1.5"
        /> < title > { title } < / title > < / svg >
    }
}
