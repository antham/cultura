use crate::config::ConfigResolver;

pub struct Shell<'a> {
    config_resolver: &'a ConfigResolver,
}

impl<'a> Shell<'a> {
    pub fn new(config_resolver: &'a ConfigResolver) -> Shell {
        Shell { config_resolver }
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
        let log_flag = if self.config_resolver.is_log_enabled() {
            "-e true"
        } else {
            ""
        };
        format!(
            r#"cultura {} daemon start
cultura {} fact generate-random"#,
            log_flag, log_flag,
        )
    }
}
