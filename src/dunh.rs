#[derive(Debug, Clone, Copy, Default)]
pub struct Line {
  number: usize,
  start_index: usize,
  end_index: usize,
}

pub fn high_err(start: usize, end: usize, raw: &str) -> String {
  high(start, end, raw, "\x1b[4m\x1b[31m")
}

pub fn high_warn(start: usize, end: usize, raw: &str) -> String {
  high(start, end, raw, "\x1b[4m\x1b[33m")
}

pub fn high_info(start: usize, end: usize, raw: &str) -> String {
  high(start, end, raw, "\x1b[4m\x1b[36m")
}

pub fn high_err_ctx(start: usize, end: usize, raw: &str, ctx_line: usize) -> String {
  high_ctx(start, end, raw, "\x1b[4m\x1b[31m", ctx_line)
}

pub fn high_warn_ctx(start: usize, end: usize, raw: &str, ctx_line: usize) -> String {
  high_ctx(start, end, raw, "\x1b[4m\x1b[33m", ctx_line)
}

pub fn high_info_ctx(start: usize, end: usize, raw: &str, ctx_line: usize) -> String {
  high_ctx(start, end, raw, "\x1b[4m\x1b[36m", ctx_line)
}

pub fn high(start: usize, end: usize, raw: &str, color: &str) -> String {
  high_ctx(start, end, raw, color, 0)
}

fn high_ctx(start: usize, end: usize, raw: &str, color: &str, ctx_line: usize) -> String {
  assert!(start <= end);
  let text = if end <= raw.len() {
    raw.to_string()
  } else {
    format!("{}{}", raw, " ".repeat(end - raw.len()))
  };

  let line_info = get_line_info(&text, start, end);
  let ctx = get_ctx(&text, &line_info, ctx_line, ctx_line);
  build_highed_text(&text, &ctx, start, end, color)
}

fn get_line_info(text: &str, start: usize, end: usize) -> Vec<Line> {
  let mut line_info = Vec::new();
  let mut current_line = Line { number: 1, start_index: 0, end_index: 0 };

  for (idx, chr) in text.char_indices() {
    if idx >= start && line_info.is_empty() {
      line_info.push(current_line);
    }

    if chr == '\n' {
      current_line.end_index = idx;
      if idx >= start && idx < end && !line_info.iter().any(|li| li.number == current_line.number) {
        line_info.push(current_line);
      }
      current_line.number += 1;
      current_line.start_index = idx + 1;
    }

    if idx >= end {
      if !line_info.iter().any(|li| li.number == current_line.number) {
        current_line.end_index = idx;
        line_info.push(current_line);
      }
      break;
    }
  }

  if line_info.is_empty() {
    current_line.end_index = text.len();
    line_info.push(current_line);
  }

  line_info
}

fn get_ctx(text: &str, line_info: &[Line], ctx_line: usize, lines_after: usize) -> Vec<Line> {
  let start_line = line_info.first().unwrap().number;
  let end_line = line_info.last().unwrap().number;

  let context_start = core::cmp::max(1, start_line.saturating_sub(ctx_line));
  let context_end = core::cmp::min(end_line + lines_after, text.lines().count());

  (context_start..=context_end)
    .map(|line_num| {
      let start_index = text.lines().take(line_num - 1).map(|l| l.len() + 1).sum();
      let end_index = start_index + text.lines().nth(line_num - 1).map_or(0, |l| l.len());
      Line { number: line_num, start_index, end_index }
    })
    .collect()
}
fn build_highed_text(text: &str, ctx: &[Line], start: usize, end: usize, color: &str) -> String {
  let reset = "\x1b[0m";
  let num_len = ctx.last().unwrap().number.to_string().len();

  let mut result = String::with_capacity(text.len() * 2);
  let mut highing = false;

  for line in ctx {
    let line_content = &text[line.start_index..line.end_index];
    let line_number = format!("{:>width$}", line.number, width = num_len);

    result.push_str(&format!("{} | ", line_number));

    let mut current_index = line.start_index;
    for chr in line_content.chars() {
      if current_index >= start && current_index < end && !highing {
        result.push_str(color);
        highing = true;
      } else if current_index >= end && highing {
        result.push_str(reset);
        highing = false;
      }
      result.push(chr);
      current_index += chr.len_utf8();
    }

    if highing {
      result.push_str(reset);
      highing = false;
    }

    result.push('\n');
  }

  result
}
