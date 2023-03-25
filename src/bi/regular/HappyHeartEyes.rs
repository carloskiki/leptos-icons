#[cfg(feature = "BiRegularHappyHeartEyes")]
use leptos::*;
#[cfg(feature = "BiRegularHappyHeartEyes")]
///This icon requires the feature `BiRegularHappyHeartEyes` to be enabled.
#[component]
pub fn HappyHeartEyes(
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
        xmlns = "http://www.w3.org/2000/svg" d = "M12 18c4 0 5-4 5-4H7s1 4 5 4z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 22c5.514 0 10-4.486 10-10S17.514 2 12 2 2 6.486 2 12s4.486 10 10 10zm0-18c4.411 0 8 3.589 8 8s-3.589 8-8 8-8-3.589-8-8 3.589-8 8-8z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "m8.535 12.634 2.05-2.083a1.485 1.485 0 0 0-.018-2.118 1.49 1.49 0 0 0-2.065-.034 1.488 1.488 0 0 0-2.067.068c-.586.6-.579 1.53.019 2.117l2.081 2.05zm7 0 2.05-2.083a1.485 1.485 0 0 0-.018-2.118 1.49 1.49 0 0 0-2.065-.034 1.488 1.488 0 0 0-2.068.067c-.586.6-.579 1.53.019 2.117l2.082 2.051z"
        /> < title > { title } < / title > < / svg >
    }
}
