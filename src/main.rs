struct WellKnown{
    flag: bool,
    ftp: u32,
    ftp_two: u32,
    ssh: u32,
    telnet: u32,
    // @TODO Add well known ports
}
impl WellKnown{
    pub fn new(flag: bool) -> Self{
        let ftp= 20;
        let ftp_two= 21;
        let ssh= 22;
        let telnet= 23;

        if flag != true{
            
        }else {
            self { flag, ftp, ftp_two, ssh, telnet }
        }


        self { flag, ftp, ftp_two, ssh, telnet }
    }
}


struct Port{
    date: Some(date),
    well_known: Some(WellKnown),
    port_number: Some(u32),
    established: Some(false),
}

fn main() {
    println!("Hello, world!");
}