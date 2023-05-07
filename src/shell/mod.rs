pub fn generate_fish_config(enable_log: bool) {
    print!(
        r#"
function fish_greeting
{}
end
"#,
        generate_commands(enable_log),
    )
}

pub fn generate_bash_config(enable_log: bool) {
    println!("{}", generate_commands(enable_log));
}

pub fn generate_zsh_config(enable_log: bool) {
    println!("{}", generate_commands(enable_log));
}

fn generate_commands(enable_log: bool) -> String {
    let log_flag = if enable_log { "-e true" } else { "" };
    format!(
        r#"cultura {} daemon start
cultura {} fact generate-random"#,
        log_flag, log_flag,
    )
}
