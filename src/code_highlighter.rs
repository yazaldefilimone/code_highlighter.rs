#[derive(Debug, Clone, Copy, Default)]
pub struct LineInfo {
  number: usize,
  start_index: usize,
  end_index: usize,
}

pub fn highlight_error(ini_idx: usize, end_idx: usize, file: &str) -> String {
  highlight(ini_idx, end_idx, file, "\x1b[4m\x1b[31m")
}

pub fn highlight_warning(ini_idx: usize, end_idx: usize, file: &str) -> String {
  highlight(ini_idx, end_idx, file, "\x1b[4m\x1b[33m")
}

pub fn highlight_error_with_context(ini_idx: usize, end_idx: usize, file: &str, lines_ctx: usize) -> String {
  highlight_context(ini_idx, end_idx, file, "\x1b[4m\x1b[31m", lines_ctx)
}

pub fn highlight_warning_with_context(ini_idx: usize, end_idx: usize, file: &str, lines_ctx: usize) -> String {
  highlight_context(ini_idx, end_idx, file, "\x1b[4m\x1b[33m", lines_ctx)
}

pub fn highlight(ini_idx: usize, end_idx: usize, file: &str, color: &str) -> String {
  highlight_context(ini_idx, end_idx, file, color, 0)
}

fn highlight_context(ini_idx: usize, end_idx: usize, file: &str, color: &str, lines_ctx: usize) -> String {
  assert!(ini_idx <= end_idx);

  let text = if end_idx <= file.len() {
    file.to_string()
  } else {
    format!("{}{}", file, " ".repeat(end_idx - file.len()))
  };

  let line_info = get_line_info(&text, ini_idx, end_idx);
  let context = get_context(&text, &line_info, lines_ctx, lines_ctx);
  build_highlighted_text(&text, &context, ini_idx, end_idx, color)
}
fn get_line_info(text: &str, ini_idx: usize, end_idx: usize) -> Vec<LineInfo> {
  let mut line_info = Vec::new();
  let mut current_line = LineInfo { number: 1, start_index: 0, end_index: 0 };

  for (idx, chr) in text.char_indices() {
    if idx >= ini_idx && line_info.is_empty() {
      line_info.push(current_line);
    }

    if chr == '\n' {
      current_line.end_index = idx;
      if idx >= ini_idx && idx < end_idx && !line_info.iter().any(|li| li.number == current_line.number) {
        line_info.push(current_line);
      }
      current_line.number += 1;
      current_line.start_index = idx + 1;
    }

    if idx >= end_idx {
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

fn get_context(text: &str, line_info: &[LineInfo], lines_ctx: usize, lines_after: usize) -> Vec<LineInfo> {
  let start_line = line_info.first().unwrap().number;
  let end_line = line_info.last().unwrap().number;

  let context_start = core::cmp::max(1, start_line.saturating_sub(lines_ctx));
  let context_end = core::cmp::min(end_line + lines_after, text.lines().count());

  (context_start..=context_end)
    .map(|line_num| {
      let start_index = text.lines().take(line_num - 1).map(|l| l.len() + 1).sum();
      let end_index = start_index + text.lines().nth(line_num - 1).map_or(0, |l| l.len());
      LineInfo { number: line_num, start_index, end_index }
    })
    .collect()
}
fn build_highlighted_text(text: &str, context: &[LineInfo], ini_idx: usize, end_idx: usize, color: &str) -> String {
  let reset = "\x1b[0m";
  let num_len = context.last().unwrap().number.to_string().len();

  let mut result = String::with_capacity(text.len() * 2);
  let mut highlighting = false;

  for line in context {
    let line_content = &text[line.start_index..line.end_index];
    let line_number = format!("{:>width$}", line.number, width = num_len);

    result.push_str(&format!("{} | ", line_number));

    let mut current_index = line.start_index;
    for chr in line_content.chars() {
      if current_index >= ini_idx && current_index < end_idx && !highlighting {
        result.push_str(color);
        highlighting = true;
      } else if current_index >= end_idx && highlighting {
        result.push_str(reset);
        highlighting = false;
      }
      result.push(chr);
      current_index += chr.len_utf8();
    }

    if highlighting {
      result.push_str(reset);
      highlighting = false;
    }

    result.push('\n');
  }

  result
}
