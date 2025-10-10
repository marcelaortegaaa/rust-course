/// For normalized inputs for units of digital information (*bytes*)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ByteUnit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
}

impl ByteUnit {
    ///Accepts multiple instances and classifies into variants
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "b" | "byte" | "bytes" => Some(ByteUnit::Bytes),
            "kb" | "kbs" | "kilobyte" | "kilobytes" => Some(ByteUnit::Kilobytes),
            "mb" | "megabyte" | "megabytes" => Some(ByteUnit::Megabytes),
            "gb" | "gigabyte" | "gigabytes" => Some(ByteUnit::Gigabytes),
            "" => Some(ByteUnit::Bytes),
            _ => None,
        }
    }

    ///Conversion rate from any unit into bytes
    pub(super) fn multiplier(self) -> u64 {
        match self {
            ByteUnit::Bytes => 1,
            ByteUnit::Kilobytes => 1_000,
            ByteUnit::Megabytes => 1_000_000,
            ByteUnit::Gigabytes => 1_000_000_000,
        }
    }

    ///Labels for displaying
    pub fn label(self) -> &'static str {
        match self {
            ByteUnit::Bytes => "bytes",
            ByteUnit::Kilobytes => "kilobytes",
            ByteUnit::Megabytes => "megabytes",
            ByteUnit::Gigabytes => "gigabytes",
        }
    }
}
