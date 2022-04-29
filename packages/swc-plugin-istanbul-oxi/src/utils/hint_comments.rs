use once_cell::sync::Lazy;
use regex::Regex as Regexp;
use swc_plugin::{
    ast::*,
    comments::{Comment, Comments, PluginCommentsProxy},
    syntax_pos::Span,
};

/// pattern for istanbul to ignore the whole file
/// This is not fully identical to original file comments
/// https://github.com/istanbuljs/istanbuljs/blob/6f45283feo31faaa066375528f6b68e3a9927b2d5/packages/istanbul-lib-instrument/src/visitor.js#L10=
/// as regex package doesn't support lookaround
static COMMENT_FILE_REGEX: Lazy<Regexp> =
    Lazy::new(|| Regexp::new(r"^\s*istanbul\s+ignore\s+(file)(\W|$)").unwrap());

/// pattern for istanbul to ignore a section
pub static COMMENT_RE: Lazy<Regexp> =
    Lazy::new(|| Regexp::new(r"^\s*istanbul\s+ignore\s+(if|else|next)(\W|$)").unwrap());

pub fn should_ignore_file(comments: &Option<&PluginCommentsProxy>, program: &Program) -> bool {
    if let Some(comments) = *comments {
        let pos = match program {
            Program::Module(module) => module.span,
            Program::Script(script) => script.span,
        };

        let validate_comments = |comments: &Option<Vec<Comment>>| {
            if let Some(comments) = comments {
                comments
                    .iter()
                    .any(|comment| COMMENT_FILE_REGEX.is_match(&comment.text))
            } else {
                false
            }
        };

        let x = vec![0];

        vec![
            comments.get_leading(pos.lo),
            comments.get_leading(pos.hi),
            comments.get_trailing(pos.lo),
            comments.get_trailing(pos.hi),
        ]
        .iter()
        .any(|c| validate_comments(c))
    } else {
        false
    }
}

pub fn lookup_hint_comments(
    comments: &Option<&PluginCommentsProxy>,
    span: Option<&Span>,
) -> Option<String> {
    if let Some(span) = span {
        let h = comments.get_leading(span.hi);
        let l = comments.get_leading(span.lo);

        if let Some(h) = h {
            let h_value = h.iter().find_map(|c| {
                if let Some(re_match) = COMMENT_RE.find_at(&c.text, 0) {
                    Some(re_match.as_str().to_string())
                } else {
                    None
                }
            });

            if let Some(h_value) = h_value {
                return Some(h_value);
            }
        }

        if let Some(l) = l {
            let l_value = l.iter().find_map(|c| {
                if let Some(re_match) = COMMENT_RE.find_at(&c.text, 0) {
                    Some(re_match.as_str().to_string())
                } else {
                    None
                }
            });

            if let Some(l_value) = l_value {
                return Some(l_value);
            }
        }
    }

    return None;
}

fn should_ignore_child(comments: &Option<&PluginCommentsProxy>, span: &Span) -> bool {
    false
}