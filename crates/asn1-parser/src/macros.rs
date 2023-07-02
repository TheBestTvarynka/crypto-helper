macro_rules! check_tag {
    (in: $reader:ident) => {
        let tag = Tag($reader.read_byte()?);

        if Self::TAG != tag {
            return Err(crate::Error::from("Invalid OctetString tag"));
        }
    };
}
