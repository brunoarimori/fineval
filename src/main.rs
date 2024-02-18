extern crate regex;
extern crate once_cell;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufRead};
// use std::path::Path;

macro_rules! regex {
  ($re:literal $(,)?) => {{
    static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
    RE.get_or_init(|| regex::Regex::new($re).unwrap())
  }};
}

#[derive(Debug, Clone)]
pub struct Entry {
  value: Option<i128>,
  mark: Option<String>,
  tag: String,
  line: u32,
}

#[derive(Debug, Clone)]
pub struct Item {
  section: String,
  title_line: u32,
  title: String,
  entries: Vec<Entry>,
  result: Result,
}

#[derive(Debug, Clone)]
pub struct Result {
  line: u32,
  value: Option<i128>,
  label: String,
}

pub struct Fin {
  file_path: String,
  current_section: Option<String>,
  buffer_lines: Vec<String>,
  all_items: Vec<Item>,
  evaluated: Vec<Item>,
  replace_lines: Vec<u32>,
}

pub trait Evaluator {
  fn new(path: String) -> Self;
  fn evaluate(item: &mut Item);
  fn traverse(&mut self);
}

pub trait FileHandler {
  fn read_pre_section(&mut self, line: String, state: &mut FileReaderState);
  fn read_pre_item(&mut self, line: String, state: &mut FileReaderState, idx: usize);
  fn read_pre_result(&mut self, line: String, state: &mut FileReaderState, idx: usize);
  fn read(&mut self);

  fn convert_value(value: i128) -> String;
  fn write(&mut self);
}

pub enum FileReaderState {
  PreSectionStart,
  PreItemStart,
  PreItemResult,
}

impl Evaluator for Fin {
  fn new(path: String) -> Self {
    Fin {
      file_path: path,
      all_items: vec![],
      evaluated: vec![],
      current_section: Option::None,
      buffer_lines: vec![],
      replace_lines: vec![]
    }
  }

  fn evaluate(item: &mut Item) {
    let mut total: i128 = 0;
    for entry in item.entries.iter() {
      total += entry.value.unwrap();
    }
    item.result.value = Some(total);
  }

  fn traverse(&mut self) {
    while self.evaluated.len() < self.all_items.len() {
      for item in self.all_items.iter_mut() {
        // check if already evaluated, by title_line and section
        let find = self.evaluated.iter().find(|&eval| (eval.title_line == item.title_line) && (eval.section == item.section));
        // item is already evaluated, skip
        if find.is_some() { continue; }
        // item is not evaluated
        // check if it has deps
        let mut dep_count: usize = 0;
        let item_section = item.section.clone();
        for entry in item.entries.iter_mut() {
          if entry.value.is_none() && entry.mark.is_some() {
            // item has dependecy, check if it is updatable
            let mark = entry.mark.as_ref().unwrap();
            let find = self.evaluated.iter().find(|&eval| (eval.result.label == mark.to_owned()) && (eval.section == item_section));
            if find.is_none() {
              dep_count += 1;
            } else {
              entry.value = find.unwrap().result.value;
            }
          }
        }
        // item has deps, skip
        if dep_count > 0 { continue; }
        // item is not evaluated and has no deps
        // evaluate item and put it on evaluated vector
        Self::evaluate(item);
        self.evaluated.push(item.to_owned());
      }
    }
  }
}


impl FileHandler for Fin {
  fn read_pre_section(&mut self, line: String, state: &mut FileReaderState) {
    let regex_start: &regex::Regex = regex!(r"!([0-9a-z]+)>$");
    let check_for_section_start = regex_start.captures(line.as_str());
    match check_for_section_start {
      Some(x) => {
        self.current_section = Some(x.get(1).map_or("", |m| m.as_str()).to_string());
        *state = FileReaderState::PreItemStart;
      },
      None => {},
    }
  }
  fn read_pre_item(&mut self, line: String, state: &mut FileReaderState, idx: usize) {
    // todo: check for section close
    let regex_close: &regex::Regex = regex!(r"!([0-9a-z]+)<$");
    if regex_close.is_match(line.as_str()) {
      *state = FileReaderState::PreSectionStart;
      return;
    }

    let regex_item_title: &regex::Regex = regex!(r"(?x)(^[A-Z\ ]+)");
    let check_for_item_title = regex_item_title.captures(line.as_str());
    match check_for_item_title {
      Some(x) => {
        // temp result
        let result = Result {
          line: 0,
          value: Option::None,
          label: "".to_string(),
        };
        let item = Item {
          title_line: idx as u32,
          entries: vec![],
          result,
          section: self.current_section.to_owned().unwrap_or("".to_string()),
          title: x.get(1).map_or("", |m| m.as_str()).to_string(),
        };
        self.all_items.push(item);
        *state = FileReaderState::PreItemResult;
      },
      None => {},
    }
  }
  fn read_pre_result(&mut self, line: String, state: &mut FileReaderState, idx: usize) {
    if line.is_empty() { return; }

    let item_idx = self.all_items.len() - 1;
    let item = self.all_items.get_mut(item_idx).unwrap();

    if line.clone().chars().nth(0).unwrap_or(' ') == '=' {
      item.result.line = idx as u32;
      for (idx, token) in line.split(" ").enumerate() {
        if idx == 0 { continue; }
        if idx == 1 {
          if token == "$".to_string() {
            item.result.value = Option::None;
            // save line, we will need to replace it on write routine
            self.replace_lines.push(item.result.line);
          } else {
            item.result.value = Some(token.replace(",", "").parse::<i128>().unwrap());
          }
          continue;
        }
        item.result.label = token.to_string().replace("#", "");
      }

      *state = FileReaderState::PreItemStart;
      return;
    }

    let mut value: Option<i128> = Option::None;
    let mut mark: Option<String> = Option::None;
    let mut tag: String = "".to_string();
    let line_num = idx as u32;

    let regex_entry_mark = regex!(r"\[([a-z0-9]+)\]");
    for (idx, token) in line.split(" ").enumerate() {
      if idx == 0 {
        if token == "$".to_string() {
          value = Option::None;
          // save line, we will need to replace it on write routine
          self.replace_lines.push(line_num);
        } else {
          value = Some(token.replace(",", "").parse::<i128>().unwrap());
        }
      } else {
        let check_for_entry_mark = regex_entry_mark.captures(token);
        match check_for_entry_mark {
          Some(x) => {
            mark = Some(x.get(1).map_or("", |m| m.as_str()).to_string());
          },
          None => {
            // not mark, assume it is tag
            tag = token.to_string();
          },
        }
      }
    }
    let entry = Entry {
      value,
      mark,
      tag,
      line: line_num,
    };
    item.entries.push(entry);
  }

  fn read(&mut self) {
    let file = File::open(self.file_path.as_str()).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut state: FileReaderState = FileReaderState::PreSectionStart;
    for (idx, line) in lines.enumerate() {
      let line_unwrap = line.unwrap();
      // save string for later write
      self.buffer_lines.push(line_unwrap.clone());
      match state {
        FileReaderState::PreSectionStart => self.read_pre_section(line_unwrap, &mut state),
        FileReaderState::PreItemStart => self.read_pre_item(line_unwrap, &mut state, idx),
        FileReaderState::PreItemResult => self.read_pre_result(line_unwrap, &mut state, idx),
      }
    }
  }

  fn convert_value(value: i128) -> String {
    let mut res = value.abs().to_string();
    if value > 1000 || value < -1000 {
      let mut res_signal = String::new();
      for (idx, val) in res.chars().rev().enumerate() {
        if idx != 0 && idx % 3 == 0 { res_signal.insert(0, ','); }
        res_signal.insert(0, val);
      }
      res = res_signal.clone();
    }
    if value > -1 { res.insert(0, '+'); }
    else { res.insert(0, '-'); }
    return res;
  }

  fn write(&mut self) {
    let mut file = OpenOptions::new().write(true).truncate(true).open(self.file_path.as_str()).unwrap();
    // read replace_lines
    for line_no in self.replace_lines.iter() {
      // find entry or result
      for item in self.evaluated.iter() {
        if item.result.line == *line_no {
          let val_str = Self::convert_value(item.result.value.unwrap());
          self.buffer_lines[*line_no as usize] = self.buffer_lines[*line_no as usize].replace("$", val_str.as_str());
        }
        for entry in item.entries.iter() {
          if entry.line == *line_no {
            let val_str = Self::convert_value(entry.value.unwrap());
            self.buffer_lines[*line_no as usize] = self.buffer_lines[*line_no as usize].replace("$", val_str.as_str());
          }
        }
      }
    }
    // write
    for str in self.buffer_lines.iter() {
      writeln!(file, "{}", str).unwrap();
    }
  }
}

fn main() {
  let file_path = std::env::args().nth(1).unwrap_or("./fin.log".to_string());
  let mut fin = Fin::new(file_path.clone());
  println!("fineval: evaluating {}", file_path);
  fin.read();
  fin.traverse();
  fin.write();
  println!("fineval: evaluation done!");
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    std::fs::remove_file("./test/fin-copy.log").unwrap_or_else(|_err| { println!("file removal failed") }) ;
    std::fs::copy("./test/fin.log", "./test/fin-copy.log").unwrap();
    let mut fin = Fin::new("./test/fin-copy.log".to_string());
    fin.read();
    fin.traverse();
    fin.write();
  }
}

