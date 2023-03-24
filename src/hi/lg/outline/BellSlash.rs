#[cfg(feature = "HiLgOutlineBellSlash")]
use leptos::*;
#[cfg(feature = "HiLgOutlineBellSlash")]
///This icon requires the feature `HiLgOutlineBellSlash` to be enabled.
#[component]
pub fn BellSlash(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.14314 17.0818C10.0799 17.1929 11.0332 17.25 11.9998 17.25C12.3306 17.25 12.6599 17.2433 12.9874 17.2301M9.14314 17.0818C7.24843 16.857 5.4214 16.4116 3.68848 15.7719C5.02539 14.2879 5.87549 12.3567 5.98723 10.2299M9.14314 17.0818C9.05019 17.3712 9 17.6797 9 18C9 19.6569 10.3431 21 12 21C13.2864 21 14.3837 20.1903 14.8101 19.0527M16.7749 16.7749L21 21M16.7749 16.7749C17.9894 16.5298 19.1706 16.1929 20.3111 15.7719C18.8743 14.177 17.9998 12.0656 17.9998 9.75V9.04919L18 9C18 5.68629 15.3137 3 12 3C9.5667 3 7.47171 4.44849 6.53026 6.53026M16.7749 16.7749L6.53026 6.53026M3 3L6.53026 6.53026"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
