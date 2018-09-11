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
    client: reqwest::Client
}

impl Bizhawk {

    pub fn new() -> Bizhawk {
        Bizhawk {
            client: reqwest::Client::new()
        }
    }

    pub fn write_u8(&self, region: MemRegion, address: u32, value: u8) -> Result<(), String> {

        if let Ok(mut result) = self.client.get(format!("http://localhost:5337/{}/WriteByte/{:X}/{:X}", region.as_static(), address, value).as_str()).send() {
            let response = result.text().unwrap_or("".to_string());
            if response != "ok".to_string() {
                return Err(response)
            }
        }

        Ok(())
    }

    pub fn read_u8(&self, region: MemRegion, address: u32) -> Result<u8, String> {

        let mut result = self.client.get(format!("http://localhost:5337/{}/ReadByte/{:X}", region.as_static(), address).as_str()).send().unwrap();
        let response = result.text().unwrap_or("".to_string());
        if response.get(0..1) == Some("R") {
            return Err(response)
        }
        Ok(u8::from_str_radix(response.as_str(), 16).unwrap())
    }

    pub fn write_u16(&self, region: MemRegion, address: u32, value: u16) -> Result<(), String> {

        if let Ok(mut result) = self.client.get(format!("http://localhost:5337/{}/WriteU16LE/{:X}/{:X}", region.as_static(), address, value).as_str()).send() {
            let response = result.text().unwrap_or("".to_string());
            if response != "ok".to_string() {
                return Err(response)
            }
        }

        Ok(())
    }

    pub fn read_u16(&self, region: MemRegion, address: u32) -> Result<u16, String> {

        let mut result = self.client.get(format!("http://localhost:5337/{}/ReadU16LE/{:X}", region.as_static(), address).as_str()).send().unwrap();
        let response = result.text().unwrap_or("".to_string());
        if response.get(0..1) == Some("R") {
            return Err(response)
        }
        Ok(u16::from_str_radix(response.as_str(), 16).unwrap())
    }

    pub fn write_u32(&self, region: MemRegion, address: u32, value: u32) -> Result<(), String> {

        if let Ok(mut result) = self.client.get(format!("http://localhost:5337/{}/WriteU32LE/{:X}/{:X}", region.as_static(), address, value).as_str()).send() {
            let response = result.text().unwrap_or("".to_string());
            if response != "ok".to_string() {
                return Err(response)
            }
        }

        Ok(())
    }

    pub fn read_u32(&self, region: MemRegion, address: u32) -> Result<u32, String> {

        let mut result = self.client.get(format!("http://localhost:5337/{}/ReadU32LE/{:X}", region.as_static(), address).as_str()).send().unwrap();
        let response = result.text().unwrap_or("".to_string());
        if response.get(0..1) == Some("R") {
            return Err(response)
        }
        Ok(u32::from_str_radix(response.as_str(), 16).unwrap())
    }

    pub fn read_slice(&self, region: MemRegion, address: u32, count: usize) -> Result<Box<[u8]>, String> {

        let mut result = self.client.get(format!("http://localhost:5337/{}/ReadByteRange/{:X}/{:X}", region.as_static(), address, count).as_str()).send().unwrap();
        let response = result.text().unwrap_or("".to_string());
        if response.get(0..1) == Some("R") {
            return Err(response)
        }
        let mut bytes = Vec::new();
        for i in 0..count {
            bytes.push(u8::from_str_radix(&response[i*2..i*2+2], 16).unwrap());
        }
        Ok(bytes.into_boxed_slice())
    }

    pub fn write_slice(&self, region: MemRegion, address: u32, data: Vec<u8>) -> Result<(), String> {

        let mut body = String::new();

        for i in data {
            body.push_str(&format!("{:02X}", i));
        }

        if let Ok(mut result) = self.client.get(format!("http://localhost:5337/{}/WriteByteRange/{:X}", region.as_static(), address).as_str()).body(body).send() {
            let response = result.text().unwrap_or("".to_string());
            if response != "ok".to_string() {
                return Err(response)
            }
        }
        Ok(())
    }


}