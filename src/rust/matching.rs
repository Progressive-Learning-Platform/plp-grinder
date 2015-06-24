use regex::Regex;

pub struct MatchResult<'a>
{
    pub start: usize,
    pub end: usize,
    pub index: usize,
    pub token_name: &'a str,
    pub valid: bool,
}

impl <'a> MatchResult<'a>
{
    pub fn parse(token_name: &str, result: Option<(usize, usize)>, index: usize) -> MatchResult
    {
        let (start, end, valid) = match result
        {
            Some((a, b)) => (a, b, true),
            None => (0, 0, false)
        };

        MatchResult{
            start: start,
            end: end,
            token_name: token_name,
            valid: valid,
            index: index}
    }
}

pub fn assemble_matchers(token_types: &Vec<(&str, &str)>) -> Vec<Regex>
{
    let mut matchers: Vec<Regex> = Vec::new();

    println!("Token Types Found:");
    for x in token_types.iter()
    {
        let name = x.0;
        let regex = x.1;
        let matcher = Regex::new(regex).unwrap();
        matchers.push(matcher);

        println!("\t{}:\t{}", name, regex);
    }

    matchers
}

pub fn find_first_match(matches: &Vec<MatchResult>) -> (bool, usize)
{
    let mut first_index: usize = 0;
    let mut start = 0;
    let mut end = 0;
    let mut match_found = false;

    for (index, result) in matches.iter().enumerate()
    {
        // Consider only valid results
        if !result.valid {continue}

        // Choose the result that starts first and ends last
        if !match_found || result.start < start || (result.start == start && result.end > end)
        {
            first_index = index;
            start = result.start;
            end = result.end;
            match_found = true;
        }
    }

    (match_found, first_index)
}
