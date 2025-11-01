#[macro_export]
macro_rules! try_parse_cmd {
    ($cmd_type:ty, $params:expr, $stdin_input:expr) => {
        if let Some(cmd) = <$cmd_type>::try_from($params)? {
            return <Cli as CliCmd<$cmd_type>>::try_from(cmd, $stdin_input);
        }
    };
}
