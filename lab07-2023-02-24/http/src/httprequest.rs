


#[derive(Debug, PartialEq)]
pub enum Method {
   Get,
   Post,
   Put,
   Delete,
   Uninitialized,
}
impl From<&str> for Method {
   fn from(s: &str) -> Method {
       match s {
           "GET" => Method::Get,
           "POST" => Method::Post,
           "PUT" => Method::Put,
           "DELETE" => Method::Delete,
           _ => Method::Uninitialized,
       }
   }
}

#[derive(Debug, PartialEq)]
pub enum Version {
   V1_1,
   V2_0,
   Uninitialized,
}

impl From<&str> for Version {
   fn from(s: &str) -> Version {
       match s {
           "HTTP/1.1" => Version::V1_1, 
           "HTTP/2.0" => Version::V2_0, 
           _ => Version::Uninitialized,
       }
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   #[test]
   fn test_version_1_1() {
       let m: Version = "HTTP/1.1".into();
       assert_eq!(m, Version::V1_1);
   }
   #[test]
   fn test_version_2_0() {
       let m: Version = "HTTP/2.0".into();
       assert_eq!(m, Version::V2_0);
   }
   #[test]
   fn test_method_get() {
       let m: Method = "GET".into();
       assert_eq!(m, Method::Get);
   }
   #[test]
   fn test_method_post() {
       let m: Method = "POST".into();
       assert_eq!(m, Method::Post);
   }
   #[test]
   fn test_method_put() {
       let m: Method = "PUT".into();
       assert_eq!(m, Method::Put);
   }
   #[test]
   fn test_method_delete() {
       let m: Method = "DELETE".into();
       assert_eq!(m, Method::Delete);
   }
}






 