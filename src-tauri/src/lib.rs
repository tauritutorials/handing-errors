use using_anyhow::using_anyhow;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    rust_error_handling::errors();

    let anyhow_result = using_anyhow();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            errors_in_commands::error_as_string,
            errors_in_commands::panics,
            errors_in_commands::panics_async,
            using_thiserror::using_thiserror,
            using_thiserror::using_thiserror_and_anyhow,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// standard rust error handing
mod rust_error_handling {
    use std::fs::File;

    pub fn errors() {
        // using_expect_and_unwrap(); // panics
        // basic_handling_with_match(); // handles the error and prints
        // let result = try_operator(); // returns a result
    }

    fn open_file() -> Result<File, std::io::Error> {
        File::open("/some/path")
    }

    // panic! unwrap expect. Simple and effective! and crashes your program
    fn using_expect_and_unwrap() {
        let file = open_file().expect("this will crash the program");
        // same thing without message
        // open_file().unwrap();
    }

    // match to gracefully handle the error, or carry on.
    fn basic_handling_with_match() {
        match open_file() {
            Ok(file) => {
                println!("all is good");
            }
            Err(e) => {
                println!("handling the error: {e}");
            }
        }
    }

    // if you don't want to handle the error yet use the `?` to bubble up.
    fn try_operator() -> Result<(), std::io::Error> {
        let file = open_file()?;

        // do other things

        Ok(())
    }
}

// anyhow
// most of the time your error types not a string.
// for functions that need to possibly bubble up multiple error types we should use anyhow.

// cd src-tauri
// cargo add anyhow

mod using_anyhow {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufReader, Read};

    pub fn using_anyhow() -> anyhow::Result<HashMap<String, serde_json::Value>> {
        let json_file = File::open("/some/path/file.json")?;
        let reader = BufReader::new(json_file);

        let data = serde_json::from_reader(reader)?;
        // because of anyhow we can use ? for both.

        Ok(data)
    }
}

mod errors_in_commands {
    use std::{fs::File, io::Read};

    // crashes program produces stacktrace
    #[tauri::command]
    pub fn panics() {
        panic!("I panic!");
    }

    // doesn't crash
    // thread 'tokio-runtime-worker' panicked at src/lib.rs:97:9:
    // I panic async!
    #[tauri::command]
    pub async fn panics_async() {
        panic!("I panic async!");
    }

    #[tauri::command]
    pub fn error_as_string() -> Result<String, String> {
        let mut file = File::open("some/path/file.txt").map_err(|e| e.to_string())?;
        let mut buff = String::new();

        file.read_to_string(&mut buff).map_err(|e| e.to_string())?;

        Ok(buff)
    }
    // pretty ugly and error messages lack context
    // Unhandled Promise Rejection: No such file or directory (os error 2)
    // would be nice if it said what file I tried.
}

// thiserror
//
// cargo add thiserror
mod using_thiserror {
    use crate::using_anyhow;
    use std::{fs::File, io};
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum MyCustomError {
        #[error(transparent)]
        File(#[from] io::Error),

        #[error("error opening {1}: {0}")]
        BetterFile(io::Error, &'static str),

        #[error(transparent)]
        Other(#[from] anyhow::Error),
    }

    // we must manually implement serde::Serialize
    impl serde::Serialize for MyCustomError {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            serializer.serialize_str(self.to_string().as_ref())
        }
    }

    #[tauri::command]
    pub fn using_thiserror() -> Result<(), MyCustomError> {
        File::open("some/path/file.txt")?;

        Ok(())
    }

    #[tauri::command]
    pub fn using_thiserror_and_anyhow() -> Result<(), MyCustomError> {
        let err = anyhow::anyhow!("Whoah");
        Err(err.into())
    }
}
