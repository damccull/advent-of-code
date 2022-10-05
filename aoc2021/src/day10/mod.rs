use unicode_segmentation::UnicodeSegmentation;

// Keep these counters:
// Total openers
// Total closers
// Openers (for each type)
// Closers (for each type)
// If the totals don't match, line is probably incomplete
// If totals do match, line is corrupt

pub fn run() {}

fn _parse_line(string: &str) -> _LineResult {
    let characters = UnicodeSegmentation::graphemes(string, true).collect::<Vec<_>>();

    let mut total_open = 0;
    let mut total_close = 0;

    let mut paren_left = 0;
    let mut paren_right = 0;

    let mut curly_left = 0;
    let mut curly_right = 0;

    let mut square_left = 0;
    let mut square_right = 0;

    let mut angle_left = 0;
    let mut angle_right = 0;

    for c in characters {
        match c {
            "(" => {
                total_open += 1;
                paren_left += 1;
            }
            ")" => {
                total_close += 1;
                paren_right += 1;
            }
            "{" => {
                total_open += 1;
                curly_left += 1;
            }
            "}" => {
                total_close += 1;
                curly_right += 1;
            }
            "[" => {
                total_open += 1;
                square_left += 1;
            }
            "]" => {
                total_close += 1;
                square_right += 1;
            }
            "<" => {
                total_open += 1;
                angle_left += 1;
            }
            ">" => {
                total_close += 1;
                angle_right += 1;
            }
            _ => {}
        }

        // If any closer is higher than opener, mark line corrupt
        if total_close > total_open
            || paren_right > paren_left
            || curly_right > curly_left
            || square_right > square_left
            || angle_right > angle_left
        {
            return _LineResult::Corrupt;
        }

        if total_close == total_open
            || paren_right != paren_left
            || curly_right != curly_left
            || square_right != square_left
            || angle_right != angle_left
            || paren_right > paren_left
            || curly_right > curly_left
            || square_right > square_left
            || angle_right > angle_left
        {
            return _LineResult::Corrupt;
        }

        if total_close != total_open
            || paren_right != paren_left
            || curly_right != curly_left
            || square_right != square_left
            || angle_right != angle_left
        {
            return _LineResult::Incomplete;
        }
    }
    _LineResult::Good
}

enum _LineResult {
    Good,
    Corrupt,
    Incomplete,
}
