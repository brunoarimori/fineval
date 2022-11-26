// LINE
//   value
//   mark
//   tag
// RESULT
//   value
//   label
// DEP
//   line
//   mark
// ITEM
//   line_start
//   title
//   LINE[]
//   RESULT

#[derive(Debug, Clone)]
struct Entry {
  value: Option<i128>,
  mark: Option<String>,
  tag: String,
  line: u32,
}

#[derive(Debug, Clone)]
struct Item {
  title_line: u32,
  title: String,
  entries: Vec<Entry>,
  result: Result,
}

#[derive(Debug, Clone)]
struct Result {
  line: u32,
  value: Option<i128>,
  label: String,
}

pub struct Placeholder {
  all_items: Vec<Item>,
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;

  fn evaluate(item: &mut Item) {
    let mut total: i128 = 0;
    for entry in item.entries.iter() {
      total += entry.value.unwrap();
    }
    item.result.value = Some(total);
  }

  fn get_items() -> Vec<Item> {
    // lines 11 to 15
    let test1_line1 = Entry {
      value: Some(10),
      mark: Option::None,
      tag: "hello1".to_string(),
      line: 12,
    };
    let test1_line2 = Entry {
      value: Option::None,
      mark: Some("test2".to_string()),
      tag: "hello2".to_string(),
      line: 13,
    };
    let test1_line3 = Entry {
      value: Some(10),
      mark: Option::None,
      tag: "hello3".to_string(),
      line: 14,
    };
    let test1_result = Result {
      line: 15,
      value: Option::None,
      label: "test1".to_string(),
    };
    let test1_item = Item {
      title_line: 11,
      title: "TEST ONE".to_string(),
      entries: vec![test1_line1.clone(), test1_line2.clone(), test1_line3.clone()],
      result: test1_result,
    };
    
    // lines 16 to 20
    let test2_line1 = Entry {
      value: Some(10),
      mark: Option::None,
      tag: "hello1".to_string(),
      line: 17,
    };
    let test2_line2 = Entry {
      value: Some(10),
      mark: Option::None,
      tag: "hello2".to_string(),
      line: 18,
    };
    let test2_line3 = Entry {
      value: Some(-15),
      mark: Option::None,
      tag: "hello3".to_string(),
      line: 19,
    };
    let test2_result = Result {
      line: 20,
      value: Option::None,
      label: "test2".to_string(),
    };
    let test2_item = Item {
      title_line: 16,
      title: "TEST TWO".to_string(),
      entries: vec![test2_line1.clone(), test2_line2.clone(), test2_line3.clone()],
      result: test2_result,
    };

    // lines 20 to 25
    let test3_line1 = Entry {
      value: Some(10),
      mark: Option::None,
      tag: "hello1".to_string(),
      line: 20,
    };
    let test3_line2 = Entry {
      value: Some(10),
      mark: Option::None,
      tag: "hello2".to_string(),
      line: 21,
    };
    let test3_line3 = Entry {
      value: Option::None,
      // mark: Some("test3".to_string()),
      mark: Some("test2".to_string()),
      tag: "hello3".to_string(),
      line: 22,
    };
    let test3_line4 = Entry {
      value: Option::None,
      mark: Some("test2".to_string()),
      tag: "hello4".to_string(),
      line: 23,
    };
    let test3_line5 = Entry {
      value: Option::None,
      mark: Some("test1".to_string()),
      tag: "hello5".to_string(),
      line: 24,
    };
    let test3_result = Result {
      line: 25,
      value: Option::None,
      label: "test3".to_string(),
    };
    let test3_item = Item {
      title_line: 20,
      title: "TEST THREE".to_string(),
      entries: vec![test3_line1.clone(), test3_line2.clone(), test3_line3.clone(), test3_line4.clone(), test3_line5.clone()],
      result: test3_result,
    };

    // return vec![test1_item, test2_item, test3_item];
    return vec![test3_item, test2_item, test1_item];
  }

  /*
  #[test]
  fn check0() {
    let mut items = get_items();
    calculate(&mut items[1]);
    println!("item result total: {}", items[1].result.value.unwrap());
  }
  #[test]
  fn check1() {
    // iterate array
    // if it has 0 deps
    //   add to discovered
    // else
    // iterate mark



    /*
    // let stack: Vec<Item> = vec![];
    for item in all_items.iter() {
      println!("item.title: {}", item.title);
      println!("item.deps.len(): {}", item.deps.len());
      if item.deps.len() > 0 {
        for dep in item.deps.iter() {
          println!("dep.line: {}", dep.line);
          // go to adjacent nodes
          all_items.iter().find(|it| it.result.label == dep.mark);
        } 
      }
    }
    */
    /*
    procedure DFS_iterative(G, v) is
        let S be a stack
        S.push(v)
        while S is not empty do
            v = S.pop()
            if v is not labeled as discovered then
                label v as discovered
                for all edges from v to w in G.adjacentEdges(v) do 
                    S.push(w)
    */
    let all_items = get_items();
    let mut discovered: Vec<Item> = vec![];
    let mut traversal: Vec<Item>;
    
    println!("begin depleting all_items");
    // while all_items.len() > 0 {
    for curr_root in all_items.iter() {
      // let curr_root = all_items.pop().unwrap();
      println!("current root title is {}, line {}", curr_root.title, curr_root.title_line);
      traversal = vec![curr_root.to_owned()];
      while traversal.len() > 0 {
        println!("traversal length is {}", traversal.len());
        let curr_item = traversal.pop().unwrap();
        println!("current item title is {}, line {}", curr_item.title, curr_item.title_line);
        let find = discovered.iter().find(|&disc| disc.title_line == curr_item.title_line);
        if find.is_some() { continue }; // item will just be popped from traversal
        if curr_item.deps.len() > 0 {
          for dep in curr_item.deps.iter() {
            // find deps
            let curr_label = all_items.iter().find(|it| it.result.label == dep.mark).unwrap();
            traversal.push(curr_label.to_owned());
          }
        } else {
          // item has no dependency
          // calculate it
          println!("calculating item with title {} and line {}", curr_item.title, curr_item.title_line);
          // push it to discovered
          discovered.push(curr_item);
        }
      }
    }
  }
*/
    /*
    procedure DFS_iterative(G, v) is
        let S be a stack
        S.push(v)
        while S is not empty do v = S.pop()
            if v is not labeled as discovered then
                label v as discovered
                for all edges from v to w in G.adjacentEdges(v) do 
                    S.push(w)
    */

  // evaluate all deps 0
  // remove deps from items
  // evaluate all deps 0
  // #[test]
  /*
  fn check2() {
    let mut all_items = get_items();
    let mut evaluated: Vec<Item> = vec![];
    while evaluated.len() < all_items.len() {
      // first run: if no deps, evaluate
      println!("[1] running evaluate, evaluated len {}", evaluated.len());
      for item in all_items.iter_mut() {
        if item.deps.len() > 0 {
          println!("{} deps found in item with title {}", item.deps.len(), item.title);
          continue;
        }
        println!("no deps found in item with title {}", item.title);
        calculate(item);
        evaluated.push(item.to_owned());
      }
      // second run: check evaluated, remove deps 
      println!("[2] running dep removal, evaluated len {}", evaluated.len());
      for item in all_items.iter_mut() {
        if item.deps.len() < 1 { continue }
        // for (idx, dep) in item.deps.iter_mut().enumerate()
        for idx in 0..(item.deps.len() - 1) {
          let dep = &item.deps[idx];
          let find = evaluated.iter().find(|&eval| eval.result.label == dep.mark);
          if find.is_none() {
            println!("!!! item with label {} not evaluated yet", dep.mark);
            continue;
          } // not evaluated yet, go to next dep
          let evaluated_label = &find.unwrap().result.label;
          let evaluated_result = &find.unwrap().result.value.unwrap();
          println!("iterating mutably item.entries of item with title {}", item.title);
          for entry in item.entries.iter_mut() {
            println!("entry with tag {} being processed", entry.tag);
            if entry.mark.is_none() { continue }
            println!("entry has mark {}", entry.mark.as_ref().unwrap());
            if entry.mark.as_ref().unwrap() == evaluated_label {
              println!("mark {}", entry.mark.as_ref().unwrap());
              entry.value = Some(evaluated_result.to_owned());
            }
          }
          println!("-- removing dep with mark {} on item with title {}", item.deps[idx].mark, item.title);
          item.deps.remove(idx);
        }
      }
    }
    for item in all_items.iter() {
      println!("title {}", item.title);
      println!("result {}", item.result.value.unwrap());
    }
  }
  */
  
  #[test]
  fn check3() {
    let mut all_items = get_items();
    let mut evaluated: Vec<Item> = vec![];
    println!("\n\nSTARTING TRAVERSAL");
    while evaluated.len() < all_items.len() {
      println!("[1] running evaluate, evaluated len {}", evaluated.len());
      for item in all_items.iter_mut() {
        // check if already evaluated
        let find = evaluated.iter().find(|&eval| eval.title_line == item.title_line);
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
            let find = evaluated.iter().find(|&eval| eval.result.label == mark.to_owned());
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
        evaluate(item);
        evaluated.push(item.to_owned());
      }
      println!("RUN COMPLETED\n\n");
    }
    // after all items evaluated, check values
    for item in all_items.iter() {
      println!("title {}", item.title);
      println!("result {}", item.result.value.unwrap());
    }
  }

}

