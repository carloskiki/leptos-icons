#[cfg(feature = "BiRegularCookie")]
use leptos::*;
#[cfg(feature = "BiRegularCookie")]
///This icon requires the feature `BiRegularCookie` to be enabled.
#[component]
pub fn Cookie(
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
        "M21.598 11.064a1.006 1.006 0 0 0-.854-.172A2.938 2.938 0 0 1 20 11c-1.654 0-3-1.346-3.003-2.937.005-.034.016-.136.017-.17a.998.998 0 0 0-1.254-1.006A2.963 2.963 0 0 1 15 7c-1.654 0-3-1.346-3-3 0-.217.031-.444.099-.716a1 1 0 0 0-1.067-1.236A9.956 9.956 0 0 0 2 12c0 5.514 4.486 10 10 10s10-4.486 10-10c0-.049-.003-.097-.007-.16a1.004 1.004 0 0 0-.395-.776zM12 20c-4.411 0-8-3.589-8-8a7.962 7.962 0 0 1 6.006-7.75A5.006 5.006 0 0 0 15 9l.101-.001a5.007 5.007 0 0 0 4.837 4C19.444 16.941 16.073 20 12 20z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "12.5" cy = "11.5" r = "1.5"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "8.5" cy = "8.5" r = "1.5"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "7.5" cy = "12.5" r = "1.5"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "15.5" cy = "15.5" r = "1.5"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "10.5" cy = "16.5" r = "1.5"
        /> < title > { title } < / title > < / svg >
    }
}
