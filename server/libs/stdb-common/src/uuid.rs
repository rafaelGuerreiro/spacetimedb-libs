use spacetimedb::ReducerContext;

/// Using String because we can't provide a custom Uuid SpacetimeType that can be used as primary_key.
pub type Uuid = String;

pub trait UuidExt {
    /// Random uuid v4
    fn new_uuid_v4(&self) -> Uuid;

    /// Timestamp based uuid v7
    fn new_uuid_v7(&self) -> Uuid;
}

impl UuidExt for ReducerContext {
    fn new_uuid_v4(&self) -> Uuid {
        let bytes = inner_new_uuid_v4(|| self.random());
        uuid_to_string(bytes)
    }

    fn new_uuid_v7(&self) -> Uuid {
        let millis = (self.timestamp.to_micros_since_unix_epoch() / 1000) as u64;
        let bytes = inner_new_uuid_v7(millis, || self.random());
        uuid_to_string(bytes)
    }
}

fn inner_new_uuid_v4<R>(mut rng: R) -> [u8; 16]
where
    R: FnMut() -> u8,
{
    let mut uuid_bytes = [0u8; 16];
    for byte in &mut uuid_bytes {
        *byte = rng();
    }

    // Set version to 4 and variant to RFC4122
    uuid_bytes[6] = (uuid_bytes[6] & 0x0f) | 0x40;
    uuid_bytes[8] = (uuid_bytes[8] & 0x3f) | 0x80;

    uuid_bytes
}

fn inner_new_uuid_v7<R>(timestamp_millis: u64, mut rng: R) -> [u8; 16]
where
    R: FnMut() -> u8,
{
    let mut uuid_bytes = [0u8; 16];
    let timestamp_millis = u64::to_be_bytes(timestamp_millis << 16);

    // First 48 bits are allocated to timestamp
    for index in 0..6 {
        uuid_bytes[index] = timestamp_millis[index];
    }

    // Next are random
    for index in 6..16 {
        uuid_bytes[index] = rng();
    }

    // Set version to 7 and variant same as uuidv4
    uuid_bytes[6] = (uuid_bytes[6] & 0x0f) | 0x70;
    uuid_bytes[8] = (uuid_bytes[8] & 0x3f) | 0x80;

    uuid_bytes
}

fn uuid_to_string(uuid_bytes: [u8; 16]) -> Uuid {
    format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        u32::from_be_bytes([uuid_bytes[0], uuid_bytes[1], uuid_bytes[2], uuid_bytes[3]]),
        u16::from_be_bytes([uuid_bytes[4], uuid_bytes[5]]),
        u16::from_be_bytes([uuid_bytes[6], uuid_bytes[7]]),
        u16::from_be_bytes([uuid_bytes[8], uuid_bytes[9]]),
        u64::from_be_bytes([
            0,
            0,
            uuid_bytes[10],
            uuid_bytes[11],
            uuid_bytes[12],
            uuid_bytes[13],
            uuid_bytes[14],
            uuid_bytes[15],
        ])
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_new_uuid_v4() {
        let mut random = 0u8..255;
        let uuid_bytes = inner_new_uuid_v4(move || random.next().unwrap());

        // Check version and variant
        assert_eq!(uuid_bytes[6] & 0xf0, 0x40); // Version 4
        assert_eq!(uuid_bytes[8] & 0xc0, 0x80); // Variant RFC4122

        assert_eq!(uuid_to_string(uuid_bytes), "00010203-0405-4607-8809-0a0b0c0d0e0f");
    }

    #[test]
    fn test_inner_new_uuid_v7() {
        let timestamp_millis = 1752115008844;
        let mut random = 0u8..255;
        let uuid_bytes = inner_new_uuid_v7(timestamp_millis, move || random.next().unwrap());

        // Check version and variant
        assert_eq!(uuid_bytes[6] & 0xf0, 0x70); // Version 7
        assert_eq!(uuid_bytes[8] & 0xc0, 0x80); // Variant RFC4122

        assert_eq!(uuid_to_string(uuid_bytes), "0197f231-554c-7001-8203-040506070809");
    }
}
