#[cfg(feature = "CgEventbrite")]
use leptos::*;
#[cfg(feature = "CgEventbrite")]
///This icon requires the feature `CgEventbrite` to be enabled.
#[component]
pub fn Eventbrite(
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
        "M14.6734 17.4378C12.3057 18.5419 9.5858 17.9679 7.85444 16.2039L21.8647 9.67074C21.7105 9.02969 21.4902 8.39446 21.2008 7.77376C18.8667 2.76836 12.9169 0.602811 7.91152 2.93687C2.90612 5.27092 0.740567 11.2207 3.07462 16.2261C5.40868 21.2315 11.3585 23.3971 16.3639 21.063C19.016 19.8263 20.8708 17.5746 21.6815 14.986H17.3424C16.7484 16.0235 15.8393 16.8941 14.6734 17.4378ZM9.60199 6.5621C7.23436 7.66615 5.92572 10.1185 6.16403 12.5786L16.4208 7.79583C14.6894 6.03194 11.9696 5.45805 9.60199 6.5621Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
