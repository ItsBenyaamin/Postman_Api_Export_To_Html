use std::fs;
extern crate serde_json;
use serde_json::Value;

fn main() {
    //TODO get file from cli
    //TODO check if file ext is '.json'

    let file_content = fs::read_to_string("/home/graymind75/Desktop/postman_export.json").expect("cant read file!");

    let result = serde_json::from_str(&file_content);

    if result.is_ok() {

        let base_json : Value = result.unwrap();

        let first_part_of_html = format!(r#"<html><head><title>{}</title><meta charset="utf-8"></head><body>"#, &base_json["info"]["name"].as_str().unwrap().to_string());

        let mut html_body = String::new();
        html_body.push_str(&first_part_of_html);

        let items = base_json["item"].as_array().unwrap();
        if items.len() > 0 {

            for i in 0..items.len() {
                let item = items[i].as_object().unwrap();

                let _item_name = item["name"].as_str().unwrap().to_string();

                if item.contains_key("item") {
                    let folder_parsed_result = parse_folder_object(&item);
                    html_body.push_str(folder_parsed_result.as_str());
                }else if item.contains_key("request"){
                    let request_parsed_result = parse_request_object(&item);
                    html_body.push_str(request_parsed_result.as_str());
                }
            }
        }else {
            panic!("there is no request in file!");
        }

        html_body.push_str("</body></html>");

        fs::write("result.html", html_body).unwrap();

    }else {
        panic!("cant parse file - file content must be JsonObject type!");
    }

}

fn parse_folder_object(item: &serde_json::Map<String, Value>) -> String {
    let mut folder_as_html = String::new();

    if item.contains_key("name") {
        let folder_name= &item["name"].as_str().unwrap().to_string();
        let folder_txt = format!("<br/><h2>{}</h2>", &folder_name);
        folder_as_html.push_str(&folder_txt);
    }

    if item.contains_key("description") {
        let folder_description = &item["description"].as_str().unwrap().to_string();
        let formatted_folder_description = format!("<br/>Description: {}", &folder_description);
        folder_as_html.push_str(&formatted_folder_description);
    }

    if item.contains_key("item") {
        let items = &item["item"].as_array().unwrap();
        if item.len() > 0 {
            for i in 0..items.len() {
                let current_item = &items[i].as_object().unwrap();

                if current_item.contains_key("request") {
                    let request_parse_result = parse_request_object(&current_item);
                    folder_as_html.push_str(&request_parse_result.as_str());
                }else if current_item.contains_key("item") {
                    let folder_parse_result = parse_folder_object(&current_item);
                    folder_as_html.push_str(&folder_parse_result.as_str());
                }
            }
        }
    }

    folder_as_html.push_str("<hr/>");

    folder_as_html
}

fn parse_request_object(item : &serde_json::Map<String, Value>) -> String {
    let mut request_result = String::new();

    if item.contains_key("name") {
        let req_name = &item["name"].as_str().unwrap().to_string();
        let formatted_name = format!("<h3>Request: {}</h3>", req_name);
        request_result.push_str(&formatted_name);
    }

    if item.contains_key("request") {
        let request = item["request"].as_object().unwrap();

        if request.contains_key("method") {
            let req_method = request["method"].as_str().unwrap().to_string();
            let formatted_method =  format!("Method: {}", req_method);
            request_result.push_str(&formatted_method);
        }

        if request.contains_key("url") {
            let req_url = request["url"]["raw"].as_str().unwrap();
            let formatted_url = format!(" ---- Url: {}", req_url);
            request_result.push_str(&formatted_url);
        }

        if request.contains_key("description") {
            let req_desc = request["description"].as_str().unwrap();
            let formatted_description = format!("<br/>Description: {}", req_desc);
            request_result.push_str(&formatted_description);
        }

        if request.contains_key("header") {
            let header_arr = request["header"].as_array().unwrap();
            if header_arr.len() > 0 {
                request_result.push_str("<br/>   <strong>Header:</strong>");
                for i in 0..header_arr.len() {
                    let current_header = header_arr[i].as_object().unwrap();

                    let h_key = current_header["key"].as_str().unwrap().to_string();
                    let h_value = current_header["value"].as_str().unwrap().to_string();

                    let formatted_header = format!("<br/>   -- Key: {} - Value: {}", h_key, h_value);
                    request_result.push_str(&formatted_header);

                    if current_header.contains_key("description") {
                        let mut header_desc = current_header["description"].as_str().unwrap().to_string();
                        header_desc = format!("<br/>   -- Description: {}<br/>", &header_desc);
                        request_result.push_str(&header_desc);
                    }else {
                        request_result.push_str("<br/>");
                    }
                }

            }
        }

        if request["url"].as_object().unwrap().contains_key("query") {
            let queries = request["url"]["query"].as_array().unwrap();
            if queries.len() > 0 {
                request_result.push_str("<br/><h3>----- Query Strings: </h3>");
                for i in 0..queries.len() {
                    let current_query = queries[i].as_object().unwrap();

                    let formatted_key = current_query["key"].as_str().unwrap().to_string();

                    let mut formatted_desc = String::new();
                    if current_query.contains_key("description") {
                        formatted_desc = current_query["description"].as_str().unwrap().to_string();
                    }
                    let formatted_query = format!("------------ Key: {} - Description: {}<br/>", &formatted_key, &formatted_desc);
                    request_result.push_str(&formatted_query);

                }
            }
        }
    }

    request_result.push_str("<br/>_____________________________________________________________________________________<br/>");

    request_result
}











