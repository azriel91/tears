use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    str::FromStr,
    time::Duration,
};

use leptos::{
    component,
    control_flow::For,
    either::Either,
    hydration::{AutoReload, HydrationScripts},
    prelude::{
        event_target_value, signal, ClassAttribute, CollectView, ElementChild, Get,
        GlobalAttributes, IntoView, LeptosOptions, OnAttribute, PropAttribute, RwSignal, Signal,
        Write,
    },
    view,
};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, RoutingProgress, A},
    StaticSegment,
};
use tears::{Mood, Suggestion, Trust};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

const PAGE_CLASSES: &str = "\
    bg-slate-950 \
    text-slate-100 \
    \
    h-dvh \
    w-dvw \
    p-8 \
    \
    flex \
    flex-col \
    justify-start \
    \
    text-lg \
";

const H1_CLASSES: &str = "\
    font-bold \
    font-mono \
    text-4xl \
";

const NAV_CLASSES: &str = "\
    flex \
    pb-8 \
";

const NAV_SPACER_CLASSES: &str = "\
    grow \
";

const MAIN_CLASSES: &str = "\
";

/// Alternatively:
///
/// ```rust,ignore
/// const HOMEPAGE_CLASSES: &str = "\
///     gap-4 \
///     grid \
///     grid-cols-1 \
///     lg:grid-cols-2 \
/// ";
/// ```
const HOMEPAGE_CLASSES: &str = "\
    gap-4 \
    flex \
    flex-col \
    lg:flex-row \
    lg:justify-center \
    lg:flex-wrap \
    lg:*:grow \
    lg:*:flex-1 \
";

const INPUTS_DIV_CLASSES: &str = "\
    bg-slate-900 \
    rounded-lg \
    p-8 \
    first:pt-4 \
    divide-y \
";

const INPUT_PANEL_CLASSES: &str = "\
    py-4 \
";

const FIELD_CLASSES: &str = "\
    mb-4 \
";

const FIELD_NAME_CLASSES: &str = "\
    pr-4 \
    font-bold \
    text-2xl \
";

const FIELD_DESC_CLASSES: &str = "\
";

const DESCRIPTION_LABEL_CLASSES: &str = "\
    font-bold \
    py-2 \
";

const DESCRIPTION_CLASSES: &str = "\
    py-2 \
";

const TRUST_LI_CLASSES: &str = "\
    pb-4 \
";

const RADIO_NAME_CLASSES: &str = "\
    inline-block \
    w-20 \
    px-2 \
    font-bold \
";

const MOOD_WRAPPER_CLASSES: &str = "\
    inline-block \
    w-fit \
    rounded-lg \
    ring-offset-2 \
    ring-offset-slate-950 \
    has-[:focus]:ring-2 \
    has-[:active]:ring-2 \
    has-[:focus]:ring-blue-500 \
    has-[:active]:ring-blue-500 \
";

const MOOD_RADIO_CLASSES: &str = "\
    appearance-none \
    focus-visible:outline-none \
";

const MOOD_LABEL_CLASSES: &str = "\
    inline-block \
    w-36 \
    py-1 \
    font-bold \
    text-center \
    \
    bg-slate-300 \
    hover:bg-slate-200 \
    has-[:checked]:hover:bg-slate-200 \
    has-[:active]:bg-slate-400 \
    \
    border \
    has-[:checked]:bg-slate-400 \
    has-[:checked]:border-slate-600 \
    text-slate-700 \
    \
    first-of-type:rounded-s-lg \
    last-of-type:rounded-e-lg \
";

const SUGGESTION_DIV_CLASSES: &str = "\
    bg-slate-800 \
    text-slate-100 \
    \
    min-h-40 \
    p-4 \
    rounded-lg \
    shadow-[inset_0_2px_4px_0_rgba(0,0,0,0.3)] \
    \
    overflow-scroll \
";

const SUGGESTION_DIV_PLACEHOLDER_CLASSES: &str = "\
    opacity-75 \
    italic \
    select-none \
";

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let site_prefix = option_env!("SITE_PREFIX").unwrap_or("");

    let (is_routing, set_is_routing) = signal(false);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/tears.css"/>

        // sets the document title
        <Title text="tears • azriel.im"/>

        // content for this welcome page
        <Router set_is_routing>
            <div class=PAGE_CLASSES>
                <div class="routing-progress">
                    <RoutingProgress is_routing max_time=Duration::from_millis(250)/>
                </div>
                <nav class=NAV_CLASSES>
                    <h1 class=H1_CLASSES>"💧 tears"</h1>
                    <div class=NAV_SPACER_CLASSES />
                    <A
                        href="https://github.com/azriel91/tears"
                        target="_blank"
                    >"🐙 github"</A>
                </nav>
                <main class=MAIN_CLASSES>
                    <Routes fallback=RouterFallback>
                        <Route path=StaticSegment(site_prefix) view=HomePage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[component]
fn RouterFallback() -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let pathname = move || location.pathname.get();

    view! {
        <p>"Path not found: " {pathname}</p>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let suggestions = suggestions_map();

    let trust = RwSignal::new(None::<Trust>);
    let mood = RwSignal::new(None::<Mood>);
    let suggestion = Signal::derive(move || {
        let trust = trust.get();
        let mood = mood.get();

        trust
            .zip(mood)
            .and_then(|(trust, mood)| suggestions.get(&(trust, mood)))
            .cloned()
    });

    view! {
        <div class=HOMEPAGE_CLASSES>
            <Inputs trust mood />
            <SuggestionDiv suggestion />
        </div>
    }
}

/// Default suggestions
fn suggestions_map() -> HashMap<(Trust, Mood), Suggestion> {
    let mut suggestions = HashMap::with_capacity(32);
    suggestions.insert(
        (Trust::Absent, Mood::_01_Anguished),
        Suggestion {
            action: "Stay away",
            description: "As a \"stranger\", your presence pressurizes the person, \
                and may aggravate them, even when your motive is pure.\n\
                \n\
                It may be best to find someone whom they already trust.",
        },
    );
    suggestions.insert(
        (Trust::Absent, Mood::_02_Closed),
        Suggestion {
            action: "Stay away",
            description: "Leave a gift if you must (e.g. chocolate), but your \
                presence pressurizes the person.\n\
                \n\
                If they accept the gift in your  absence, then that may be the \
                beginning of trust.",
        },
    );
    suggestions.insert(
        (Trust::Absent, Mood::_03_Cautious),
        Suggestion {
            action: "Occasionally ask if they want something",
            description: "If you are sure the person wants something (that \
                isn't harmful), ask \"do you want ____\"?\n\
                \n\
                Make sure the conversation is paced such that they are able to \
                handle it.\n\
                \n\
                Don't ask why, don't require an answer -- provide a way \"out\" \
                (e.g. \"you don't have to answer\"). Asking such questions is \
                perceived as \"justify yourself\", and may cause them to hate \
                you (which they may not vocalize).",
        },
    );
    suggestions.insert(
        (Trust::Absent, Mood::_04_Unsettled),
        Suggestion {
            action: "Ask, \"would you like to say anything?\", then wait.",
            description: "Just listen, don't problem solve -- you haven't established \
                trust with the person to do so.\n\
                \n\
                At this stage, you may have some rational conversation, but \
                nothing that would introduce too much emotional pressure.\n\
                \n\
                Be ready to leave them alone if that is what they want (they \
                may not say it).",
        },
    );
    suggestions.insert(
        (Trust::Absent, Mood::_05_Calm),
        Suggestion {
            action: "Be calm / hopeful.",
            description: "Find some gentle fun -- the person is ready to explore.\n\
                \n\
                Be ready to leave them alone if that is what they want (they \
                may not say it).",
        },
    );
    suggestions.insert(
        (Trust::Absent, Mood::_06_Hopeful),
        Suggestion {
            action: "Enjoy yourselves.",
            description: "Make new happy memories -- the person needs them.\n\
                \n\
                This is your chance to help them believe life can be good.",
        },
    );

    suggestions.insert(
        (Trust::Present, Mood::_01_Anguished),
        Suggestion {
            action: "Be fully present with them",
            description: "Simply sit quietly with them and allow them to \
                grieve.\n\
                \n\
                Any more than that may overwhelm the person.",
        },
    );
    suggestions.insert(
        (Trust::Present, Mood::_02_Closed),
        Suggestion {
            action: "Remain at a small distance",
            description: "Leave a gift if you have one, to show that they are \
                still someone you care for; but allow a little distance -- \
                your presence may feel like pressure to the person in the \
                moment.\n\
                \n\
                Distance allows them to settle, proximity allows them to feel \
                cared for.",
        },
    );
    suggestions.insert(
        (Trust::Present, Mood::_03_Cautious),
        Suggestion {
            action: "Occasionally ask if they want something",
            description: "If you are sure the person wants something (that \
                isn't harmful), ask \"do you want ____\"?\n\
                \n\
                Make sure the conversation is paced such that they are able to \
                handle it.\n\
                \n\
                Don't ask why, don't require an answer -- provide a way \"out\" \
                (e.g. \"you don't have to answer\"). Asking such questions is \
                perceived as \"justify yourself\", and may cause them to hate \
                you (which they may not vocalize).",
        },
    );
    suggestions.insert(
        (Trust::Present, Mood::_04_Unsettled),
        Suggestion {
            action: "Ask, \"would you like to say anything?\", then wait.",
            description: "Listen, and if it feels right you may ask, \"Would \
                you like some help with it?\" (if you are able to help).\n\
                \n\
                At this stage, you may have some rational conversation, but \
                nothing that would introduce too much emotional pressure.\n\
            ",
        },
    );
    suggestions.insert(
        (Trust::Present, Mood::_05_Calm),
        Suggestion {
            action: "Be calm / hopeful.",
            description: "Find some gentle fun -- the person is ready to explore.",
        },
    );
    suggestions.insert(
        (Trust::Present, Mood::_06_Hopeful),
        Suggestion {
            action: "Enjoy yourselves.",
            description: "Make new happy memories -- the person needs them.\n\
                \n\
                Help them remember life can be good.",
        },
    );

    suggestions
}

#[component]
fn Inputs(trust: RwSignal<Option<Trust>>, mood: RwSignal<Option<Mood>>) -> impl IntoView {
    view! {
        <div class=INPUTS_DIV_CLASSES>
            <TrustInput trust />
            <MoodInput mood />
        </div>
    }
}

#[component]
fn TrustInput(trust: RwSignal<Option<Trust>>) -> impl IntoView {
    let trust_on_input =
        move |ev| *trust.write() = Trust::from_str(event_target_value(&ev).as_str()).ok();

    view! {
        <div class=INPUT_PANEL_CLASSES>
            <p class=FIELD_CLASSES>
                <span class=FIELD_NAME_CLASSES>"Trust"</span>
                <span class=FIELD_DESC_CLASSES>"Whether the person trusts you."</span>
            </p>
            <ul>
                <li class=TRUST_LI_CLASSES>
                    <label for="trust_radio_absent">
                        <input
                            type="radio"
                            name="trust_radio"
                            id="trust_radio_absent"
                            on:input=trust_on_input
                            prop:value=Trust::Absent.to_string()
                        />
                        <span class=RADIO_NAME_CLASSES>{Trust::Absent.to_string()}</span>
                        <span class=DESCRIPTION_CLASSES>"The person has not initiated a conversation with me recently."</span>
                    </label>
                </li>
                <li class=TRUST_LI_CLASSES>
                    <label for="trust_radio_present">
                        <input
                            type="radio"
                            name="trust_radio"
                            id="trust_radio_present"
                            on:input=trust_on_input
                            prop:value=Trust::Present.to_string()
                        />
                        <span class=RADIO_NAME_CLASSES>{Trust::Present.to_string()}</span>
                        <span class=DESCRIPTION_CLASSES>"The person has initiated a conversation with me recently, with no obligation."</span>
                    </label>
                </li>
            </ul>
        </div>
    }
}

#[component]
fn MoodInput(mood: RwSignal<Option<Mood>>) -> impl IntoView {
    let mood_on_input =
        move |ev| *mood.write() = Mood::from_str(event_target_value(&ev).as_str()).ok();

    view! {
        <div class=INPUT_PANEL_CLASSES>
            <p class=FIELD_CLASSES>
                <span class=FIELD_NAME_CLASSES>"Mood"</span>
                <span class=FIELD_DESC_CLASSES>"How the person feels."</span>
            </p>
            <div class=MOOD_WRAPPER_CLASSES>
                {
                    Mood::iter()
                        .map(|mood| {
                            let rank = mood.rank();
                            let mood_radio_id = format!("mood_radio_{mood}");

                            view! {
                                <label
                                    for=mood_radio_id.clone()
                                    class=MOOD_LABEL_CLASSES
                                >
                                    <input
                                        type="radio"
                                        class=MOOD_RADIO_CLASSES
                                        name="mood_radio"
                                        id=mood_radio_id.clone()
                                        on:input=mood_on_input
                                        prop:value=move || mood.to_string()
                                    />
                                    <span>
                                        {rank.to_string()}
                                        <br />
                                        {mood.to_string()}
                                    </span>
                                </label>
                            }
                        })
                        .collect_view()
                }
            </div>
            { move || {
                let mood = mood.get();
                match mood {
                    Some(mood) => {
                        Either::Left(view! {
                            <p class=DESCRIPTION_CLASSES>
                                <span class=DESCRIPTION_LABEL_CLASSES>"Symptoms: "</span>
                                {mood.symptoms()}
                            </p>
                            <p class=DESCRIPTION_CLASSES>{mood.summary()}</p>
                            <p class=DESCRIPTION_CLASSES>{mood.description()}</p>
                        })
                    }
                    None => Either::Right(view! {})
                }
            }}
        </div>
    }
}

#[component]
fn SuggestionDiv(suggestion: Signal<Option<Suggestion>>) -> impl IntoView {
    let placeholder_classes = move || {
        if suggestion.get().is_some() {
            SUGGESTION_DIV_PLACEHOLDER_CLASSES
        } else {
            "hidden"
        }
    };
    view! {
        <div class=SUGGESTION_DIV_CLASSES>
            {move || {
                match suggestion.get() {
                    Some(suggestion) => {
                        Either::Left(view! {
                            <div>
                                <p class=DESCRIPTION_CLASSES>
                                    <span class=DESCRIPTION_LABEL_CLASSES>"Action: "</span>
                                    {suggestion.action().to_string()}
                                </p>
                                <For
                                    each=move || {
                                        suggestion.description()
                                            .split("\n\n")
                                            .map(ToString::to_string)
                                            .collect::<Vec<_>>()
                                    }
                                    key=string_hash
                                    children=|line: String| view! { <p class=DESCRIPTION_CLASSES>{line.to_string()}</p> }
                                />
                            </div>
                        })
                    }
                    None => {
                        Either::Right(view! {
                            <span class=placeholder_classes>
                                "Please select if the person trusts you in this moment, and the mood they are in."
                            </span>
                        })
                    }
                }
            }}
        </div>
    }
}

fn string_hash(s: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
