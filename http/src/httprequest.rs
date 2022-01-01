use std::collections::HashMap;

#[derive(Debug,PartialEq)]
pub enum Method{
    Get,
    Post,
    Uninitialized,
}
//手动实现From接口
impl From<&str> for Method {
    fn from(s:&str)->Method{
        match s {
            "GET"=>Method::Get,
            "POST" => Method::Post,
            _=>Method::Uninitialized,    
        }
    }
}
#[derive(Debug,PartialEq)]
pub enum Version{
    V1_1,
    V2_0,
    Uninitialized,
}
//手动实现From接口
impl From<&str> for Version {
    fn from(s:&str)->Version{
        match s {
            "HTTP/1.1"=>Version::V1_1,
            _=>Version::Uninitialized,    
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum Resource{
    //请求路径
    Path(String),
}

#[derive(Debug)]
pub struct  HttpRequest{
    pub method:Method,
    pub version:Version,
    pub path:Resource,
    pub head:HashMap<String,String>,
    pub msg_body:String,
}

impl From<String> for HttpRequest {
    fn from(req:String)->Self{
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers=HashMap::new();
        let mut parsed_msg_body="";
        for line in req.lines(){
            if line.contains("HTTP"){
                let (method,resource,version) = process_req_line(req);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version=version;
            }
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_met_into(){
        let m:Method = "GET".into();
        assert_eq!(m,Method::Get);
    }

    #[test]
    fn test_ver_into(){
        let m:Version = "HTTP/1.1".into();
        assert_eq!(m,Version::V1_1);
    }
}