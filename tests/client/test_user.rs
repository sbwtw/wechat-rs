use wechat::WeChatClient;
use wechat::client::WeChatUser;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";
const OPENID: &'static str = "ozJS1syaqn5ztglMsr8ceH8o2zCQ";


#[test]
fn test_user_get() {
    let client = WeChatClient::new(APPID, SECRET);
    let user = WeChatUser::new(&client);

    let res = user.get(OPENID);
    assert!(res.is_ok());
}

#[test]
fn test_user_get_with_lang() {
    let client = WeChatClient::new(APPID, SECRET);
    let user = WeChatUser::new(&client);

    let res = user.get_with_lang(OPENID, "zh_CN");
    assert!(res.is_ok());
}

#[test]
fn test_user_update_remark() {
    let client = WeChatClient::new(APPID, SECRET);
    let user = WeChatUser::new(&client);

    let res = user.update_remark(OPENID, "test user");
    assert!(res.is_ok());
}

#[test]
fn test_user_get_followers_with_no_next_openid() {
    let client = WeChatClient::new(APPID, SECRET);
    let user = WeChatUser::new(&client);

    let res = user.get_followers("");
    assert!(res.is_ok());
}

#[test]
fn test_user_get_followers_with_next_openid() {
    let client = WeChatClient::new(APPID, SECRET);
    let user = WeChatUser::new(&client);

    let res = user.get_followers(OPENID);
    assert!(res.is_ok());
}

#[test]
fn test_user_get_group_id() {
    let client = WeChatClient::new(APPID, SECRET);
    let user = WeChatUser::new(&client);

    let res = user.get_group_id(OPENID);
    assert!(res.is_ok());
}