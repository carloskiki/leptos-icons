#[cfg(feature = "CgLayoutPin")]
use leptos::*;
#[cfg(feature = "CgLayoutPin")]
///This icon requires the feature `CgLayoutPin` to be enabled.
#[component]
pub fn LayoutPin(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M16.9307 4.01587H14.7655C14.3582 2.84239 13.2428 2 11.9307 2C10.6185 2 9.50313 2.84239 9.09581 4.01587H6.93066C5.27381 4.01587 3.93066 5.35901 3.93066 7.01587V9.21205C2.80183 9.64283 2 10.7357 2 12.0159C2 13.296 2.80183 14.3889 3.93066 14.8197V17.0159C3.93066 18.6727 5.27381 20.0159 6.93066 20.0159H9.08467C9.48247 21.2064 10.6064 22.0645 11.9307 22.0645C13.255 22.0645 14.3789 21.2064 14.7767 20.0159H16.9307C18.5875 20.0159 19.9307 18.6727 19.9307 17.0159V14.8446C21.095 14.4322 21.929 13.3214 21.929 12.0159C21.929 10.7103 21.095 9.5995 19.9307 9.18718V7.01587C19.9307 5.35901 18.5875 4.01587 16.9307 4.01587ZM5.93066 14.8687V17.0159C5.93066 17.5682 6.37838 18.0159 6.93066 18.0159H9.11902C9.54426 16.8761 10.6427 16.0645 11.9307 16.0645C13.2187 16.0645 14.3171 16.8761 14.7423 18.0159H16.9307C17.4829 18.0159 17.9307 17.5682 17.9307 17.0159V14.8458C16.7646 14.4344 15.929 13.3227 15.929 12.0159C15.929 10.709 16.7646 9.59732 17.9307 9.18597V7.01587C17.9307 6.46358 17.4829 6.01587 16.9307 6.01587H14.7543C14.338 7.17276 13.2309 8 11.9307 8C10.6304 8 9.52331 7.17276 9.10703 6.01587H6.93066C6.37838 6.01587 5.93066 6.46358 5.93066 7.01587V9.16302C7.13193 9.55465 8 10.6839 8 12.0159C8 13.3479 7.13193 14.4771 5.93066 14.8687Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
