use reqwest;
use strum::AsStaticRef;

//Memory regions
#[derive(AsStaticStr)]
pub enum MemRegion {
    BIOS,
    CartRAM,
    CombinedWRAM,
    EWRAM,
    HRAM,
    IWRAM,
    OAM,
    PALRAM,
    ROM,
    SRAM,
    SystemBus,
    VRAM,
    WRAM
}

pub struct Bizhawk {
    client: reqwest::Client,
    port: u16
}

impl Bizhawk {
    pub fn new(port: u16) -> Bizhawk {
        Bizhawk {
            client: reqwest::Client::new(),
            port
        }
    }

    pub fn send_command(&self, command: &str) -> Result<(), String> {
        if let Ok(mut result) = self.client.get(format!("http://localhost:{}/{}", self.port, command).as_str()).send() {
            let response = result.text().unwrap_or_default();
            if response != "ok" {
                return Err(response)
            }
        } else {
            return Err("Failed to send command to bizhawk.".to_string())
        }
        Ok(())
    }

    pub fn send_command_and_get_response(&self, command: &str) -> Result<String, String> {
        if let Ok(mut result) = self.client.get(format!("http://localhost:{}/{}", self.port, command).as_str()).send() {
            let response = result.text().unwrap_or_default();
            if response.get(0..1) == Some("R") {
                return Err(response)
            }
            return Ok(response)
        } else {
            return Err("Failed to send command to bizhawk".to_string())
        }
    }

    pub fn write_u8(&self, region: MemRegion, address: u32, value: u8) -> Result<(), String> {
        self.send_command(format!("{}/WriteByte/{:X}/{:X}", region.as_static(), address, value).as_str())
    }

    pub fn write_u16(&self, region: MemRegion, address: u32, value: u16) -> Result<(), String> {
        self.send_command(format!("{}/WriteU16LE/{:X}/{:X}", region.as_static(), address, value).as_str())
    }

    pub fn write_u32(&self, region: MemRegion, address: u32, value: u32) -> Result<(), String> {
        self.send_command(format!("{}/WriteU32LE/{:X}/{:X}", region.as_static(), address, value).as_str())
    }

    pub fn write_u16_be(&self, region: MemRegion, address: u32, value: u16) -> Result<(), String> {
        self.send_command(format!("{}/WriteU16BE/{:X}/{:X}", region.as_static(), address, value).as_str())
    }

    pub fn write_u32_be(&self, region: MemRegion, address: u32, value: u32) -> Result<(), String> {
        self.send_command(format!("{}/WriteU32BE/{:X}/{:X}", region.as_static(), address, value).as_str())
    }

    pub fn write_slice(&self, region: MemRegion, address: u32, data: &[u8]) -> Result<(), String> {
        let mut body = String::new();
        for i in data {
            body.push_str(&format!("{:02X}", i));
        }
        if let Ok(mut result) = self.client.get(format!("http://localhost:{}/{}/WriteByteRange/{:X}", self.port, region.as_static(), address).as_str()).body(body).send() {
            let response = result.text().unwrap_or_default();
            if response != "ok" {
                return Err(response)
            }
        }
        Ok(())
    }

    pub fn read_u8(&self, region: MemRegion, address: u32) -> Result<u8, String> {
        match self.send_command_and_get_response(format!("{}/ReadByte/{:X}", region.as_static(), address).as_str()) {
            Ok(byte) => Ok(u8::from_str_radix(byte.as_str(), 16).unwrap()),
            Err(error) => Err(error)
        }
    }

    pub fn read_u16(&self, region: MemRegion, address: u32) -> Result<u16, String> {
        match self.send_command_and_get_response(format!("{}/ReadU16LE/{:X}", region.as_static(), address).as_str()) {
            Ok(word) => Ok(u16::from_str_radix(word.as_str(), 16).unwrap()),
            Err(error) => Err(error)
        }
    }

    pub fn read_u32(&self, region: MemRegion, address: u32) -> Result<u32, String> {
        match self.send_command_and_get_response(format!("{}/ReadU32LE/{:X}", region.as_static(), address).as_str()) {
            Ok(dword) => Ok(u32::from_str_radix(dword.as_str(), 16).unwrap()),
            Err(error) => Err(error)
        }
    }

    pub fn read_u16_be(&self, region: MemRegion, address: u32) -> Result<u16, String> {
        match self.send_command_and_get_response(format!("{}/ReadU16BE/{:X}", region.as_static(), address).as_str()) {
            Ok(word) => Ok(u16::from_str_radix(word.as_str(), 16).unwrap()),
            Err(error) => Err(error)
        }
    }

    pub fn read_u32_be(&self, region: MemRegion, address: u32) -> Result<u32, String> {
        match self.send_command_and_get_response(format!("{}/ReadU32BE/{:X}", region.as_static(), address).as_str()) {
            Ok(dword) => Ok(u32::from_str_radix(dword.as_str(), 16).unwrap()),
            Err(error) => Err(error)
        }
    }

    pub fn read_slice(&self, region: MemRegion, address: u32, count: usize) -> Result<Box<[u8]>, String> {
        match self.send_command_and_get_response(format!("{}/ReadByteRange/{:X}/{:X}", region.as_static(), address, count).as_str()) {
            Ok(response) => {
                let mut bytes = Vec::new();
                for i in 0..count {
                    bytes.push(u8::from_str_radix(&response[i*2..i*2+2], 16).unwrap());
                }
                Ok(bytes.into_boxed_slice())
            },
            Err(error) => Err(error)
        }
    }

    pub fn read_slice_custom(&self, message: String, count: usize) -> Result<Box<[u8]>, String> {
        match self.send_command_and_get_response(format!("ReadByteRange/{}", message).as_str()) {
            Ok(response) => {
                let mut bytes = Vec::new();
                for i in 0..count {
                    bytes.push(u8::from_str_radix(&response[i*2..i*2+2], 16).unwrap());
                }
                Ok(bytes.into_boxed_slice())
            },
            Err(error) => Err(error)
        }
    }

    pub fn save_state(&self, name: &str) -> Result<(), String> {
        self.send_command(format!("savestate/States/{}.State", name).as_str())
    }

    pub fn load_state(&self, name: &str) -> Result<(), String> {
        self.send_command(format!("loadstate/States/{}.State", name).as_str())
    }

    pub fn load_rom(&self, name: &str) -> Result<(), String> {
        self.send_command(format!("loadrom/Roms/{}", name).as_str())
    }

    pub fn pause(&self) -> Result<(), String> {
        self.send_command("pause")
    }

    pub fn play(&self) -> Result<(), String> {
        self.send_command("play")
    }

    pub fn stop_drawing(&self) -> Result<(), String> {
        self.send_command("stopdrawing")
    }

    pub fn start_drawing(&self) -> Result<(), String> {
        self.send_command("startdrawing")
    }

    pub fn mute(&self) -> Result<(), String> {
        self.send_command("mute")
    }

    pub fn unmute(&self) -> Result<(), String> {
        self.send_command("unmute")
    }

    pub fn unthrottle(&self, frames: u32) -> Result<(), String> {
        if frames == 0 {
            self.send_command("unthrottle")
        } else {
            self.send_command(format!("unthrottle/{}", frames).as_str())
        }
    }

    pub fn throttle(&self) -> Result<(), String> {
        self.send_command("throttle")
    }

    pub fn frameadvance(&self) -> Result<(), String> {
        self.send_command("frameadvance")
    }

    pub fn framerewind(&self) -> Result<(), String> {
        self.send_command("framerewind")
    }

    pub fn clear_screen(&self, color: u32) -> Result<(), String> {
        self.send_command(format!("clearscreen/{:06X}", color).as_str())
    }

    pub fn framecount(&self) -> Result<u32, String> {
        match self.send_command_and_get_response("framecount") {
            Ok(dword) => Ok(u32::from_str_radix(dword.as_str(), 10).unwrap()),
            Err(error) => Err(error)
        }
    }

    pub fn get_rom_name(&self) -> Result<String, String> {
        if let Ok(mut result) = self.client.get(format!("http://localhost:{}/GetROMName", self.port).as_str()).send() {
            let response = result.text().unwrap_or_default();
            Ok(response)
        } else {
            Err("Failed to get rom name".to_string())
        }
    }
}