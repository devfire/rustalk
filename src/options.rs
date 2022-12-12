use clap::Parser;

// NOTE the arg_required_else_help parameter. It forces a default help when no CLI inputs are passed.
// It is undocumented but does exist, see here
// https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs#L19
#[derive(Parser)]
#[command(author, version, about, arg_required_else_help = true, long_about = None)]
pub struct Cli {
    ///Multicast address to join or send data to
    #[arg(short, long,required = true)]
    pub(crate) multicast: std::net::Ipv4Addr,

    // NOTE: if a default value is needed use value_name = "DEFAULT_VALUE"
    /// Sets a custom log file
    #[arg(short, long,required = true)]
    pub(crate) port: u16,

    /// Username to use
    #[arg(short, long,required = true)]
    pub(crate) username: String,

    /// IP of the local interface to send/receive data
    #[arg(short, long,required = true)]
    pub(crate) ip: std::net::Ipv4Addr,
}