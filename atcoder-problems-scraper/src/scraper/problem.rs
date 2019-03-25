use crate::scraper::{get_html, ATCODER_HOST};
use crate::Problem;
use scraper::{Html, Selector};

pub(crate) fn scrape_problems_from_html(html: &str, contest_id: &str) -> Option<Vec<Problem>> {
    Html::parse_document(html)
        .select(&Selector::parse("tbody").unwrap())
        .next()?
        .select(&Selector::parse("tr").unwrap())
        .map(|tr| {
            let selector = Selector::parse("td").unwrap();
            let mut tds = tr.select(&selector);
            let problem = tds.nth(1)?;
            let id = problem
                .select(&Selector::parse("a").unwrap())
                .next()?
                .value()
                .attr("href")?
                .rsplit('/')
                .next()?
                .to_owned();
            let title = problem.text().next()?.to_owned();
            Some(Problem {
                id: id,
                contest_id: contest_id.to_owned(),
                title,
            })
        })
        .collect()
}

pub fn scrape_problems(contest_id: &str) -> Result<Vec<Problem>, String> {
    let url = format!("{}/contests/{}/tasks", ATCODER_HOST, contest_id);
    let page_html = get_html(&url)?;
    scrape_problems_from_html(&page_html, contest_id).ok_or_else(|| format!("Error: {}", url))
}