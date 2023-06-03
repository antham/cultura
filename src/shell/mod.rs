pub struct Shell {}

impl Shell {
    pub fn new() -> Shell {
        Shell {}
    }

    pub fn generate_fish_config(self) {
        print!(
            r#"
function fish_greeting
{}
end
"#,
            self.generate_commands(),
        )
    }

    pub fn generate_bash_config(self) {
        println!("{}", self.generate_commands());
    }

    pub fn generate_zsh_config(self) {
        println!("{}", self.generate_commands());
    }

    fn generate_commands(self) -> String {
        r#"cultura daemon start
cultura fact generate-random"#
            .to_string()
    }
}
