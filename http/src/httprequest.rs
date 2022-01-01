use std::{collections::HashMap, os::windows::process};

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
    pub resource:Resource,
    pub header:HashMap<String,String>,
    pub msg_body:String,
}
/**
 * 处理http请求
 */
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
            }else if line.contains(":") { //处理header行
                let (key,value) = process_header_line(req);
                parsed_headers.insert(key,value);
                
            }else if line.len()==0 {//处理空行
                
            }else {//处理消息体
                parsed_msg_body = process_msg_body(line);
            }
        }
        //将得到的信息组成HttpRequest类型返回
        HttpRequest{
            method:parsed_method,
            version:parsed_version,
            resource:parsed_resource,
            header:parsed_headers,
            msg_body:parsed_msg_body.to_string(),
        }
    }
}
//实现处理请求头函数
fn process_header_line(s：&str)->(Method,Resource,Version){
    let mut words = s.split_whitespace();
    let  method = words.next().unwrap();
    let mut resource = words.next().unwrap();
    let mut version = words.next().unwrap();
    (method.into(),Resource::Path(resource.to_string()),version.into(),)
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