//TODO: correct the length impl
fn is_valid_and_len<const MAX_LEN: u16, const ABSOLUTE_MAX_LEN>(str: &str) -> Option<u16> {
       if str.len() * 2 > MAX_LEN as usize || str.len() * 2 > ABSOLUTE_MAX_LEN as usize {
           return None;
       }

       let mut len = 0;

       for char in str.chars() {
           if char > '\u{FFFF}' {
               len += 2;
           } else {
               len += 1;
           }
       }

       if len > MAX_LEN {
           None
       } else {
           Some(len)
       }
   }