#[derive(RustcDecodable)]
pub struct Args {
    pub cmd_add: bool,
    pub arg_ip: Option<String>,
    pub arg_host: Option<String>,
    pub cmd_delete: bool,
    pub arg_host_ip: Option<String>,
    pub cmd_list: bool,
    pub flag_h: bool,
    pub flag_v: bool,
}

impl Args {
    pub fn new() -> Args {
        Args {
            cmd_add: false,
            arg_ip: None,
            arg_host: None,
            cmd_delete: false,
            arg_host_ip: None,
            cmd_list: false,
            flag_h: false,
            flag_v: false,
        }
    }
}
