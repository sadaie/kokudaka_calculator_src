mod class;
mod kansuji;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::document;
use stdweb::web::event::ClickEvent;
use stdweb::web::html_element::InputElement;
use stdweb::web::HtmlElement;
use crate::kansuji::IntoKansuji;

const ERROR_MESSAGE: &'static str = "ほ…なにかがおかしいのです…";

fn error_mapper<T>(_: T) -> &'static str {
    ERROR_MESSAGE
}

fn calculate_kokudaka(closeness: u64, num_fans: u64, bnp_rank: u64) -> u64 {
    (closeness + num_fans)
        .checked_sub(bnp_rank * 10_000)
        .unwrap_or((closeness + num_fans) / 10)
}

fn get_values() -> Result<(String, u64, u64, u64), &'static str> {
    let name_input: InputElement = document()
        .get_element_by_id("name")
        .ok_or(ERROR_MESSAGE)?
        .try_into()
        .map_err(error_mapper)?;
    let name = name_input.raw_value();
    let closeness_input: InputElement = document()
        .get_element_by_id("closeness")
        .ok_or(ERROR_MESSAGE)?
        .try_into()
        .map_err(error_mapper)?;
    let closeness: u64 = closeness_input.raw_value().parse().map_err(error_mapper)?;
    let num_fans_input: InputElement = document()
        .get_element_by_id("num-fans")
        .ok_or(ERROR_MESSAGE)?
        .try_into()
        .map_err(error_mapper)?;
    let num_fans: u64 = num_fans_input.raw_value().parse().map_err(error_mapper)?;
    let bnp_rank_input: InputElement = document()
        .get_element_by_id("bnp-rank")
        .ok_or(ERROR_MESSAGE)?
        .try_into()
        .map_err(error_mapper)?;
    let bnp_rank: u64 = bnp_rank_input.raw_value().parse().map_err(error_mapper)?;

    Ok((name, closeness, num_fans, bnp_rank))
}

fn set_texts(name: &str, kokudaka: u64) {
    let result_title: HtmlElement = document()
        .get_element_by_id("result-title")
        .unwrap()
        .try_into()
        .unwrap();
    let result_paragraph: HtmlElement = document()
        .get_element_by_id("result-paragraph")
        .unwrap()
        .try_into()
        .unwrap();
    let close_button: HtmlElement = document()
        .get_element_by_id("close-button")
        .unwrap()
        .try_into()
        .unwrap();

    let title = format!(
        "{}さんの石高",
        if name.is_empty() {
            "名無しの家臣"
        } else {
            name
        }
    );
    result_title.set_text_content(&title);

    let kansujinized = kokudaka.into_kansuji();
    let class = class::find_closest_han(kokudaka);
    let result_text = match class {
        class::Class::Over => format!("{}石なのです 加賀の前田さん家よりわんだほーなのです！", kansujinized),
        class::Class::CloseTo(h) => format!("{}石なのです {}の{}さん家くらいなのです", kansujinized, h.han_name, h.family_name),
        class::Class::Less => format!("{}石なのです", kansujinized),
    };

    result_paragraph.set_text_content(&result_text);

    let button = format!(r#"<a href="https://twitter.com/intent/tweet?text={}は{}%0a@sadaie_p%20さんから%0a%0a&url=https://kokudaka.matsuri-hi.me/&hashtags=あなたの石高,TC徳川まつり,imas_ml_tc" onClick="window.open(encodeURI(decodeURI(this.href)), 'tweetwindow', 'width=650, height=470, personalbar=0, toolbar=0, scrollbars=1, sizable=1'); return false;" rel="nofollow" class="uk-button bg-matsuri-iro uk-margin-left">
ツイートする
</a>"#, title, result_text);
    let _ = close_button.insert_html_after(&button);
}

fn main() {
    stdweb::initialize();

    let submit_button: InputElement = document()
        .get_element_by_id("submit-button")
        .unwrap()
        .try_into()
        .unwrap();
    submit_button.add_event_listener(|_: ClickEvent| match get_values() {
        Ok((name, closeness, num_fans, bnp_rank)) => {
            set_texts(&name, calculate_kokudaka(closeness, num_fans, bnp_rank))
        }
        Err(m) => println!("{:?}", m),
    });

    stdweb::event_loop();
}
