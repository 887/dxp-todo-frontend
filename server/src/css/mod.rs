use is_executable::IsExecutable;

pub static STYLE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/frontend/css/");
pub static STYLE_SHEET_OUTPUT_DIR: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/frontend/static/css/");
pub static STYLES: [&str; 2] = ["style.css", "layout.css"];

async fn run_bundler_command(
    sheet_file_name: &str,
) -> Result<std::process::Output, std::io::Error> {
    let mut exe = match std::env::current_exe() {
        Ok(res) => res,
        Err(err) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("No executable {err}"),
            ))
        }
    };

    //remove old exe
    exe.pop();
    //push the one we want
    exe.push("frontend_css_builder");
    let p = exe.as_path();
    if !(exe.is_executable()) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "File not executable",
        ));
    }

    //https://stackoverflow.com/questions/21011330/how-do-i-invoke-a-system-command-and-capture-its-output
    let style_sheet = STYLE_DIR.to_string() + sheet_file_name;
    let sheet_path = Path::new(&style_sheet);
    let style_sheet_output = STYLE_SHEET_OUTPUT_DIR.to_string() + sheet_file_name;
    let sheet_output_path = Path::new(&style_sheet_output);
    Command::new(p)
        .arg("--style-sheet")
        .arg(sheet_path)
        .arg("--output")
        .arg(sheet_output_path)
        .output()
        .await
}

pub async fn run_css_builder(stylesheet_name: &str) -> Result<String, String> {
    let result = match run_bundler_command(stylesheet_name).await {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    if !result.status.success() {
        match String::from_utf8(result.stderr) {
            Ok(stderr_output) => return Err(stderr_output),
            Err(err) => return Err(err.to_string()),
        };
    }

    let css_maybe = String::from_utf8(result.stdout);
    match css_maybe {
        Ok(css) => Ok(css),
        Err(err) => Err(err.to_string()),
    }
}
