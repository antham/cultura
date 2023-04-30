pub fn generate_fish_config() {
    print!(
        r#"
function fish_greeting
   cultura daemon start
   cultura fact generate-random
end
"#
    )
}

pub fn generate_bash_config() {
    print!(
        r#"
cultura daemon start
cultura fact generate-random
"#
    )
}

pub fn generate_zsh_config() {
    print!(
        r#"
cultura daemon start
cultura fact generate-random
"#
    )
}
