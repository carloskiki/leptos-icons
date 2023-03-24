#[cfg(feature = "BiRegularRightTopArrowCircle")]
use leptos::*;
#[cfg(feature = "BiRegularRightTopArrowCircle")]
///This icon requires the feature `BiRegularRightTopArrowCircle` to be enabled.
#[component]
pub fn RightTopArrowCircle(
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
        "M12.006 2.007a9.927 9.927 0 0 0-7.071 2.922c-3.898 3.899-3.898 10.243 0 14.142 1.885 1.885 4.396 2.923 7.071 2.923s5.187-1.038 7.071-2.923c3.898-3.898 3.898-10.242 0-14.142a9.928 9.928 0 0 0-7.071-2.922zm5.657 15.649c-1.507 1.507-3.517 2.337-5.657 2.337s-4.15-.83-5.657-2.337c-3.118-3.119-3.118-8.194 0-11.313 1.507-1.507 3.516-2.336 5.657-2.336s4.15.829 5.657 2.336c3.118 3.119 3.118 8.194 0 11.313z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "m11.467 11.125-3.214 3.214 1.414 1.414 3.214-3.214 2.125 2.124V9H9.343z" /> <
        title > { title } < / title > < / svg >
    }
}
