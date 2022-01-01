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
    pub resource:Resource,
    pub headers:HashMap<String,String>,
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
                let (method,resource,version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version=version;
            }else if line.contains(":") { //处理header行
                let (key,value) = process_header_line(line);
                parsed_headers.insert(key,value);
                
            }else if line.len()==0 {//处理空行
                
            }else {//处理消息体
                parsed_msg_body = line;
            }
        }
        //将得到的信息组成HttpRequest类型返回
        HttpRequest{
            method:parsed_method,
            version:parsed_version,
            resource:parsed_resource,
            headers:parsed_headers,
            msg_body:parsed_msg_body.to_string(),
        }
    }
}
fn process_req_line(s:&str)->(Method,Resource,Version){
    //将传入字符串按空白分成多个单词
    let mut words = s.split_whitespace();
    let  method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (method.into(),Resource::Path(resource.to_string()),version.into(),)
}
//实现处理请求头函数
fn process_header_line(s:&str)->(String,String){
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }
    (key,value)
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

    #[test]
    fn test_http_request(){
        let s:String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n");
        //测试结果
        let mut headers_expeced=HashMap::new();
        headers_expeced.insert("Host".into(), " localhost".into());
        headers_expeced.insert("Accept".into()," */*".into());
        headers_expeced.insert("User-Agent".into(), " curl/7.71.1".into());
        let req:HttpRequest=s.into();
        
        assert_eq!(Method::Get,req.method);
        assert_eq!(Version::V1_1,req.version);
        assert_eq!(Resource::Path("/greeting".to_string()),req.resource);
        assert_eq!(headers_expeced,req.headers);
    }
}