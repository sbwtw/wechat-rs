use std::collections::HashMap;

use rustc_serialize::json::Json;

use client::WeChatClient;
use errors::WeChatError;

#[derive(Debug, Clone)]
pub struct WeChatUser<'a> {
    client: &'a WeChatClient,
}

impl<'a> WeChatUser<'a> {

    #[inline]
    pub fn new(client: &'a WeChatClient) -> WeChatUser<'a> {
        WeChatUser {
            client: client,
        }
    }

    pub fn get(&self, openid: &str) -> Result<Json, WeChatError> {
        self.client.get("user/info", vec![("openid", openid)])
    }

    pub fn get_with_lang(&self, openid: &str, lang: &str) -> Result<Json, WeChatError> {
        self.client.get("user/info", vec![("openid", openid), ("lang", lang)])
    }

    pub fn update_remark(&self, openid: &str, remark: &str) -> Result<Json, WeChatError> {
        let data = json!({
            "openid": (openid),
            "remark": (remark),
        });
        self.client.post("user/info/updateremark", vec![], &data)
    }

    pub fn get_followers(&self, next_openid: &str) -> Result<Json, WeChatError> {
        self.client.get("user/get", vec![("next_openid", next_openid)])
    }

    pub fn get_group_id(&self, openid: &str) -> Result<u64, WeChatError> {
        let res = try!(self.client.post("groups/getid", vec![], &json!({"openid": (openid)})));
        let group_id = res.find("groupid").unwrap();
        let group_id = group_id.as_u64().unwrap();
        Ok(group_id)
    }

    pub fn get_batch(&self, user_list: &[HashMap<String, String>]) -> Result<Json, WeChatError> {
        self.client.post("user/info/batchget", vec![], &json!({"user_list": (user_list)}))
    }

    pub fn get_batch_with_lang(&self, user_list: &[String], lang: &str) -> Result<Json, WeChatError> {
        let mut users = vec![];
        for openid in user_list {
            let mut user = HashMap::new();
            user.insert("openid".to_owned(), openid.to_owned());
            user.insert("lang".to_owned(), lang.to_owned());
            users.push(user);
        }
        self.get_batch(&users)
    }
}