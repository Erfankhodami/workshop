pub fn to_utf8(c:char)->Vec<u8>{
    let mut buf=[0;4];
    let encoded=c.encode_utf8(&mut buf);
    encoded.as_bytes().to_vec()
}
