use anyhow::Context;

pub fn format_wwn(wwn: [u8; 8]) -> String {
    wwn.iter()
        .map(|value| format!("{:02x}", value))
        .collect::<Vec<_>>()
        .join(":")
}

pub fn wwn_from_string(wwn: String) -> Result<[u8; 8], anyhow::Error> {
    let a = wwn
        .split(":")
        .map(|elem| u8::from_str_radix(elem, 16))
        .collect::<Result<Vec<_>, _>>()
        .context(format!("Failed to parse wwn {}", wwn))?;
    let a: [u8; 8] = a
        .try_into()
        .map_err(|_| anyhow::Error::msg(format!("Failed to parse wwn {}", wwn)))?;
    Ok(a)
}
