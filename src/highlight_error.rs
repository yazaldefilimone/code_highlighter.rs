/// Given a complete source file, highlights an error between two indexes (in bytes).
pub fn highlight_error(ini_idx: usize, end_idx: usize, file: &str) -> String {
  // Please do NOT "improve" this by using high-order functions

  fn pad(len: usize, txt: &str) -> String {
    return format!("{}{}", " ".repeat(std::cmp::max(len - txt.len(), 0)), txt);
  }

  let color = "\x1b[4m\x1b[31m";
  let reset = "\x1b[0m";

  let mut cur_lin_idx = 0; // current line index
  let mut cur_lin_num = 0; // current line number

  let mut slc_lin_idx = 0; // slice line index
  let mut slc_lin_num = 0; // slice line number

  let mut slc_end_idx = 0; // slice end index

  let mut idx = 0;
  while idx <= file.len() {
    let chr = file[idx..].chars().nth(0).unwrap_or('\n');
    //println!("[{}] | {} {} {} {} {} {} | {}", if chr == '\n' { 'N' } else { chr }, idx, cur_lin_idx, cur_lin_num, slc_lin_idx, slc_lin_num, slc_end_idx, ini_idx);
    if idx == ini_idx {
      slc_lin_idx = cur_lin_idx;
      slc_lin_num = cur_lin_num;
    }
    if chr == '\n' {
      cur_lin_idx = idx + 1;
      cur_lin_num = cur_lin_num + 1;
      if idx >= end_idx {
        slc_end_idx = idx;
        break;
      }
    }
    idx += chr.len_utf8();
  }

  let num_len = format!("{}", cur_lin_idx + 1).len();
  let slice   = &file[slc_lin_idx .. slc_end_idx];
  let ini_idx = ini_idx - slc_lin_idx;
  let end_idx = end_idx - slc_lin_idx;

  let mut text = String::new();
  let mut newl = true;
  let mut high = false;
  let mut line = slc_lin_num;
  let mut idx = 0;
  while idx < slice.len() {
    let chr = slice[idx..].chars().nth(0).unwrap_or(' ');
    if newl {
      text.push_str(reset);
      text.push_str(&format!(" {} | ", pad(num_len, &format!("{}", line + 1))));
      if high {
        text.push_str(color);
      }
      newl = false;
    }
    if chr == '\n' {
      newl = true;
      line = line + 1;
    }
    if idx == ini_idx {
      high = true;
      text.push_str(color);
    }
    if chr == '\n' && high && end_idx - ini_idx == 1 {
      text.push(' '); // single "\n" highlight
    }
    if idx == end_idx {
      high = false;
      text.push_str(reset);
    }
    text.push(chr);
    idx += chr.len_utf8();
  }
  text.push_str(reset);

  return text;
}
