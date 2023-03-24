#[cfg(feature = "HiLgSolidSpeakerXMark")]
use leptos::*;
#[cfg(feature = "HiLgSolidSpeakerXMark")]
///This icon requires the feature `HiLgSolidSpeakerXMark` to be enabled.
#[component]
pub fn SpeakerXMark(
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
        "M13.5 4.06063C13.5 2.72427 11.8843 2.05501 10.9393 2.99996L6.43934 7.49997H4.50905C3.36772 7.49997 2.19106 8.1644 1.8493 9.40502C1.62147 10.2321 1.5 11.1024 1.5 12C1.5 12.8975 1.62147 13.7678 1.8493 14.5949C2.19106 15.8355 3.36772 16.5 4.50905 16.5H6.43934L10.9393 21C11.8843 21.9449 13.5 21.2757 13.5 19.9393V4.06063Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.7803 9.21962C17.4874 8.92673 17.0126 8.92673 16.7197 9.21962C16.4268 9.51252 16.4268 9.98739 16.7197 10.2803L18.4393 12L16.7197 13.7196C16.4268 14.0125 16.4268 14.4874 16.7197 14.7803C17.0126 15.0732 17.4874 15.0732 17.7803 14.7803L19.5 13.0606L21.2197 14.7803C21.5126 15.0732 21.9874 15.0732 22.2803 14.7803C22.5732 14.4874 22.5732 14.0125 22.2803 13.7196L20.5607 12L22.2803 10.2803C22.5732 9.98739 22.5732 9.51252 22.2803 9.21962C21.9874 8.92673 21.5126 8.92673 21.2197 9.21962L19.5 10.9393L17.7803 9.21962Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
