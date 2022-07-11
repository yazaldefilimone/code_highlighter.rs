#![allow(dead_code)]
use itertools::Itertools as _;

fn main() {
  let code = "
    for (var i = 0; i < 10; ++i) {
      console.log('ola tudo bem');
    }
  ";

  println!("Error:\n{}", highlight(5, 33, code));
}

pub fn find(text: &str, target: &str) -> usize {
  text.find(target).unwrap_or_else(|| panic!("`{}` not in `{}`.", target, text))
}

pub fn highlight(from_index: usize, to_index: usize, code: &str) -> String {
  debug_assert!(to_index >= from_index);
  debug_assert!(code.get(from_index..to_index).is_some());
  //let open = "<<<<####";
  //let close = "####>>>>";
  let open = "««««";
  let close = "»»»»";
  let open_color = "\x1b[4m\x1b[31m";
  let close_color = "\x1b[0m";
  let mut from_line = 0;
  let mut to_line = 0;
  for (i, c) in code.char_indices().filter(|(_, c)| c == &'\n').take_while(|(i, _)| i < &to_index) {
    if i < from_index {
      from_line += c.len_utf8();
    }
    to_line += c.len_utf8();
  }
  let code =
    [&code[0..from_index], open, &code[from_index..to_index], close, &code[to_index..code.len()]]
      .concat();
  let block_from_line = std::cmp::max(from_line as i64 - 3, 0) as usize;
  let block_to_line = std::cmp::min(to_line + 3, code.lines().count());
  code
    .lines()
    .enumerate()
    .skip_while(|(i, _)| i < &block_from_line)
    .take_while(|(i, _)| i < &block_to_line)
    .map(|(_, line)| line)
    .enumerate()
    .format_with("", |(i, line), f| {
      let numb = block_from_line + i;
      // TODO: An allocation of an intermediate string still occurs here
      // which is inefficient. Should figure out how to improve this.
      let rest = if numb == from_line && numb == to_line {
        [
          &line[0..find(line, open)],
          open_color,
          &line[find(line, open) + open.len()..find(line, close)],
          close_color,
          &line[find(line, close) + close.len()..line.len()],
          "\n",
        ]
        .concat()
      } else if numb == from_line {
        [&line[0..find(line, open)], open_color, &line[find(line, open)..line.len()], "\n"].concat()
      } else if numb > from_line && numb < to_line {
        [open_color, line, close_color, "\n"].concat()
      } else if numb == to_line {
        [
          &line[0..find(line, open)],
          open_color,
          &line[find(line, open)..find(line, close) + close.len()],
          close_color,
          "\n",
        ]
        .concat()
      } else {
        [line, "\n"].concat()
      };
      f(&format_args!("    {} | {}", numb, rest))
    })
    .to_string()
}
