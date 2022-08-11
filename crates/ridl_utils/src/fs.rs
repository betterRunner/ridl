use std::{
  env,
  fs::{metadata, read_dir, read_to_string},
  sync::{Arc, Mutex},
};

pub fn read_file_str_content(file_path: &str) -> String {
  let content_str = match read_to_string(file_path) {
    Ok(data) => data,
    Err(err) => {
      println!("reading file with error: {}", err);
      "".to_string()
    }
  };
  content_str
}

pub fn get_absolute_path(rpath: &str) -> String {
  let mut path = env::current_dir().unwrap();
  path.push(rpath);
  let path = path.as_os_str().to_str().unwrap().to_string();
  path
}

pub fn get_all_files_path_of_folder(folder_path: &str, postfix: &str) -> Arc<Mutex<Vec<String>>> {
  // see https://stackoverflow.com/questions/30559073/cannot-borrow-captured-outer-variable-in-an-fn-closure-as-mutable about why using `Arc` and `Mutex`
  let mut res = Arc::new(Mutex::new(Vec::<String>::new()));
  struct Iter<'s> {
    f: &'s dyn Fn(&Iter, &str) -> (),
  }
  let iter = Iter {
    f: &|iter, path| {
      let paths = read_dir(path).unwrap();
      for path in paths {
        let path = path.unwrap().path();
        let meta = metadata(path.clone()).unwrap();
        let path = path.to_str().unwrap();
        if meta.is_dir() {
          (iter.f)(iter, path);
        } else {
          res.lock().unwrap().push(String::from(path));
        }
      }
    },
  };
  (iter.f)(&iter, folder_path);
  res
}
