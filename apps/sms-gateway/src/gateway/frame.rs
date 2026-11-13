//! Length-prefixed JSON framing shared by Bluetooth (and desktop BT simulator).
//!
//! Wire format: 4-byte big-endian length + UTF-8 JSON body (max 64 KiB).

use std::io::{Read, Write};

pub const MAX_FRAME: usize = 64 * 1024;

pub fn write_frame(w: &mut impl Write, json: &str) -> Result<(), String> {
    let bytes = json.as_bytes();
    if bytes.len() > MAX_FRAME {
        return Err("frame too large".into());
    }
    let len = (bytes.len() as u32).to_be_bytes();
    w.write_all(&len).map_err(|e| e.to_string())?;
    w.write_all(bytes).map_err(|e| e.to_string())?;
    w.flush().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn read_frame(r: &mut impl Read) -> Result<String, String> {
    let mut len_buf = [0u8; 4];
    r.read_exact(&mut len_buf).map_err(|e| e.to_string())?;
    let len = u32::from_be_bytes(len_buf) as usize;
    if len > MAX_FRAME {
        return Err("frame too large".into());
    }
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf).map_err(|e| e.to_string())?;
    String::from_utf8(buf).map_err(|e| e.to_string())
}
