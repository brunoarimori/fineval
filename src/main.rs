// LINE
//   value
//   mark
//   tag
// RESULT
//   value
//   label
// ITEM
//   line_start
//   title
//   LINE[]
//   RESULT

extern crate regex;
extern crate once_cell;
use std::fs::File;
use std::io::{self, BufRead};
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
  current_section: Option<String>,
  all_items: Vec<Item>,
  evaluated: Vec<Item>,
}

pub trait Evaluator {
  fn new() -> Self;
  fn evaluate(item: &mut Item);
  fn traverse(&mut self);
}

impl Evaluator for Fin {
  fn new() -> Self {
    Fin { all_items: vec![], evaluated: vec![], current_section: Option::None }
  }

  fn evaluate(item: &mut Item) {
    let mut total: i128 = 0;
    for entry in item.entries.iter() {
      total += entry.value.unwrap();
    }
    item.result.value = Some(total);
  }

  fn traverse(&mut self) {
    println!("\n\nSTARTING TRAVERSAL");
    while self.evaluated.len() < self.all_items.len() {
      println!("[1] running evaluate, evaluated len {}", self.evaluated.len());
      for item in self.all_items.iter_mut() {
        // check if already evaluated
        let find = self.evaluated.iter().find(|&eval| eval.title_line == item.title_line);
        if find.is_some() {
          // item is already evaluated, skip
          println!("item with title {} was already evaluated, skipping", item.title);
          continue;
        }
        // item is not evaluated
        // check if it has deps
        let mut dep_count: usize = 0;
        for entry in item.entries.iter_mut() {
          if entry.value.is_none() && entry.mark.is_some() {
            // item has dependecy, check if it is updatable
            let mark = entry.mark.as_ref().unwrap();
            println!("on item {}, entry {}: checking evaluated for label {}", item.title, entry.tag, mark);
            let find = self.evaluated.iter().find(|&eval| eval.result.label == mark.to_owned());
            if find.is_none() {
              println!("evaluated item not found for this dependency");
              dep_count += 1;
            } else {
              println!("updating value for entry {}", entry.tag);
              entry.value = find.unwrap().result.value;
            }
          }
        }
        if dep_count > 0 {
          // item has deps, skip
          println!("{} deps found in item with title {}, skipping", dep_count, item.title);
          continue;
        }
        // item is not evaluated and has no deps
        println!("no deps found in item with title {}, evaluating...", item.title);
        // evaluate item and put it on evaluated vector
        Self::evaluate(item);
        self.evaluated.push(item.to_owned());
      }
      println!("RUN COMPLETED\n\n");
    }
    // after all items evaluated, check values
    for item in self.all_items.iter() {
      println!("title {}", item.title);
      println!("result {}", item.result.value.unwrap());
    }
  }
}

pub trait FileHandler {
  fn read_pre_section(&mut self, line: String, state: &mut FileReaderState);
  fn read_pre_item(&mut self, line: String, state: &mut FileReaderState, idx: usize);
  fn read_pre_result(&mut self, line: String, state: &mut FileReaderState, idx: usize);

  fn read(&mut self);
  fn write(&self);
}

pub enum FileReaderState {
  PreSectionStart,
  PreItemStart,
  PreItemResult,
}

impl FileHandler for Fin {
  fn read_pre_section(&mut self, line: String, state: &mut FileReaderState) {
    println!("on read section!!!");
    let regex_start: &regex::Regex = regex!(r"!([0-9a-z]+)>$");
    let check_for_section_start = regex_start.captures(line.as_str());
    match check_for_section_start {
      Some(x) => {
        self.current_section = Some(x.get(1).map_or("", |m| m.as_str()).to_string());
        *state = FileReaderState::PreItemStart;
      },
      None => println!("{}", line),
    }
  }
  fn read_pre_item(&mut self, line: String, state: &mut FileReaderState, idx: usize) {
    println!("on read item!!!");
    // todo: check for section close
    let regex_close: &regex::Regex = regex!(r"!([0-9a-z]+)<$");
    if regex_close.is_match(line.as_str()) {
      println!("line is closing!!!");
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
      None => println!("{}", line),
    }
  }
  fn read_pre_result(&mut self, line: String, state: &mut FileReaderState, idx: usize) {
    println!("on wait result!!! {}", line);
    if line.is_empty() {
      println!("line is empty!!!! ");
      return;
    }

    let item_idx = self.all_items.len() - 1;
    let item = self.all_items.get_mut(item_idx).unwrap();

    if line.clone().chars().nth(0).unwrap_or(' ') == '=' {
      item.result.line = idx as u32;
      for (idx, token) in line.split(" ").enumerate() {
        if idx == 0 { continue; }
        if idx == 1 {
          if token == "$".to_string() {
            item.result.value = Option::None;
          } else {
            item.result.value = Some(token.replace(",", "").parse::<i128>().unwrap());
          }
          continue;
        }
        println!("label: {}", token);
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
        println!("token: {}", token);
        if token == "$".to_string() {
          value = Option::None;
          println!("value is None");
        } else {
          value = Some(token.replace(",", "").parse::<i128>().unwrap());
          println!("value parse: {}", value.unwrap());
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
    println!("---");
    println!("value is {}", value.unwrap_or(0));
    println!("mark is {}", mark.clone().unwrap_or("None".to_string()));
    println!("tag is {}", tag);
    println!("line is {}", line_num);
    println!("---");

    let entry = Entry {
      value,
      mark,
      tag,
      line: line_num,
    };

    item.entries.push(entry);
  }

  fn read(&mut self) {
    let file = File::open("./fin.log").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut state: FileReaderState = FileReaderState::PreSectionStart;
    for (idx, line) in lines.enumerate() {
      let line_unwrap = line.unwrap();
      match state {
        FileReaderState::PreSectionStart => self.read_pre_section(line_unwrap, &mut state),
        FileReaderState::PreItemStart => self.read_pre_item(line_unwrap, &mut state, idx),
        FileReaderState::PreItemResult => self.read_pre_result(line_unwrap, &mut state, idx),
      }
    }
    for item in self.all_items.iter() {
      println!("section is {}", item.section);
      println!("title is {}", item.title);
      println!("title line is {}", item.title_line);
      for entry in item.entries.iter() {
        println!("  entry line is {}", entry.line);
        println!("  entry value is {}", entry.value.unwrap_or(0));
      }
    }
  }
  fn write(&self) {
  }
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn check3() {
    let mut fin = Fin::new();
    println!("====");
    for item in fin.evaluated.iter() {
      println!("item is evaluated as {}", item.result.value.unwrap());
    }
    println!("====");
    fin.read();
    fin.traverse();
  }
}

