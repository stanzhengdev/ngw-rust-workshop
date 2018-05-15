struct MyStruct {
    a_number: u32,
    a_string: String,
}

impl MyStruct {
    fn print_it(&self) {
        for i in 0..self.a_number {
	    println!("{}", self.a_string);
	}
    }

    fn number(&self) -> u32 {
        self.a_number
    }

    fn set_number(&mut self, n: u32) {
        self.a_number = n;
    }
}

fn connect(ip: [u8; 4], port: u16) -> Result<TcpSocket, Box<std::error::Error>> {
    // STUFF
}

    fn recv(&self, buf: &mut [u8]) -> Result<usize, Box<std::error::Error>> {
        // STUFF
    }


impl SomeTrait for MyStruct {
    // Required method definitions here!
}


//   fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error>
//   fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error>