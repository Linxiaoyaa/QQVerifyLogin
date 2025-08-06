use std::net::SocketAddr;
use axum::extract::ConnectInfo;
use axum::Json;
use serde_json::{Value, json};
use crate::mystruct::APIResult;

use reqwest::Client;
use serde_json::Value::Object;

pub async fn chk_captcha( version:String, sig:String, ticket:String, randstr:String, guid:String) ->APIResult<Value> {
    let client = Client::new();
    let body = json!({
         "com": {
             "src": 1,
             "scene": 100607,
             "platform": 2,
             "version": version,
             "unlgn": {
                 "uin": 0,
                 "sig": sig,
                 "sigType": 1,
                 "randstr": randstr,
             }
         },
         "ticket": ticket,
         "randStr": randstr,
         "appid": 2090581062
    });
    let chkcaptchares = match client.post("https://accounts.qq.com/login/limit/proxy/domain/qq110.qq.com/v3/chkcaptcha?uin=0&bkn=")
        .json(&body)
        .header("qname-service", "1935233:65536")
        .header("qname-space", "Production")
        .header("Cookie", "uin=; p_uin=; p_uid=; login_key_set_failed=AlreadyLogout")
        .send()
        .await
    {
        Ok(response) => {
            response
        },
        Err(err) => {
            tracing::error!("ChkcaptchaFailed (GUID: {}): {}", guid, err);
            return APIResult {
                code: 500,
                reqststus: "failed".to_string(),
                data: json!(null),
            };
        }
    };
    match chkcaptchares.json::<Value>().await {
        Ok(data) => {
            tracing::info!("ChkcaptchaSuccess: {:?}", data);
                APIResult {
                    code: 0,
                    reqststus: "success".to_string(),
                    data,
                }

        }
        Err(err) => {
            tracing::error!("SerializeChkcaptchaFailed (GUID: {}): {}", guid, err);
            APIResult {
                code: 500,
                reqststus: "Failed".to_string(),
                data: json!({ "error": err.to_string() }),
            }
        }
    }
}
pub async fn chk_risk( version:String, sig:String, qimei:String, randstr:String, guid:String,appid:String) ->APIResult<Value> {
    tracing::info!(version, sig, qimei, randstr, guid, appid);
    let client = Client::new();
    let body = json!({
         "com": {
    "src": 1,
    "scene": 101107,
    "platform": 2,
    "version": version,
    "unlgn": {
        "uin": 0,
        "sig": sig,
        "sigType": 1,
        "randstr": randstr,
    },
    "device": {
        "guid": guid,
        "qimei": qimei,
        "qimei36": qimei,
        "subappid": appid,
        "platform": "Android",
        "brand": "March",
        "model": "7",
        "bssid": "",
        "devInfo": ""
    }
    }});
    let chkriskres = match client.post("https://accounts.qq.com/login/limit/proxy/domain/qq110.qq.com/v3/chkrisk?uin=0&bkn=")
        .json(&body)
        .header("qname-service", "1935233:65536")
        .header("qname-space", "Production")
        .header("Cookie", "uin=; p_uin=; p_uid=; login_key_set_failed=AlreadyLogout")
        .send()
        .await
    {
        Ok(response) => {
            response
        },
        Err(err) => {
            tracing::error!("ChkriskFailed (GUID: {}): {}", guid, err);
            return APIResult {
                code: 500,
                reqststus: "failed".to_string(),
                data: json!(null),
            };
        }
    };
    match chkriskres.json::<Value>().await {
        Ok(data) => {
            tracing::info!("ChkriskSuccess: {:?}", data);
            APIResult {
                code: 0,
                reqststus: "success".to_string(),
                data,
            }

        }
        Err(err) => {
            tracing::error!("SerializeChkriskFailed (GUID: {}): {}", guid, err);
            APIResult {
                code: 500,
                reqststus: "Failed".to_string(),
                data: json!({ "error": err.to_string() }),
            }
        }
    }
}
pub async fn query_login_verify_method( version:String, sig:String, randstr:String, guid:String) ->APIResult<Value> {
    tracing::info!(version, sig, randstr, guid);
    let client = Client::new();
    let body = json!({
             "com": {
                "src": 1,
                "scene": 102805,
                "platform": 2,
                "version": version,
                "unlgn": {
                    "uin": 0,
                    "sig": sig,
                    "sigType": 1,
                    "randstr": randstr
                }
             }
    });
    let queryloginverifymethodres = match client.post("https://accounts.qq.com/login/limit/proxy/domain/qq110.qq.com/v3/queryloginverifymethod?uin=0&bkn=")
        .json(&body)
        .header("qname-service", "1935233:65536")
        .header("qname-space", "Production")
        .header("Cookie", "uin=; p_uin=; p_uid=; login_key_set_failed=AlreadyLogout")
        .send()
        .await
    {
        Ok(response) => {
            response
        },
        Err(err) => {
            tracing::error!("QueryloginverifymethodresFailed (GUID: {}): {}", guid, err);
            return APIResult {
                code: 500,
                reqststus: "failed".to_string(),
                data: json!(null),
            };
        }
    };
    match queryloginverifymethodres.json::<Value>().await {
        Ok(data) => {
            tracing::info!("QueryloginverifymethodresSuccess: {:?}", data);
            APIResult {
                code: 0,
                reqststus: "success".to_string(),
                data,
            }

        }
        Err(err) => {
            tracing::error!("SerializeQueryloginverifymethodresFailed (GUID: {}): {}", guid, err);
            APIResult {
                code: 500,
                reqststus: "Failed".to_string(),
                data: json!({ "error": err.to_string() }),
            }
        }
    }
}
pub async fn query_bound_phone( version:String, sig:String, randstr:String, guid:String) ->APIResult<Value> {
    tracing::info!(version, sig, randstr, guid);
    let client = Client::new();
    let body = json!({
             "com": {
                "src": 1,
                "scene": 103402,
                "platform": 2,
                "version": version,
                "unlgn": {
                    "uin": 0,
                    "sig": sig,
                    "sigType": 1,
                    "randstr": randstr,
                }
             }
    });
    let query_bound_phoneres = match client.post("https://accounts.qq.com/login/limit/proxy/domain/qq110.qq.com/v3/query_bound_phone?uin=0&bkn=")
        .json(&body)
        .header("qname-service", "1935233:65536")
        .header("qname-space", "Production")
        .header("Cookie", "uin=; p_uin=; p_uid=; login_key_set_failed=AlreadyLogout")
        .send()
        .await
    {
        Ok(response) => {
            response
        },
        Err(err) => {
            tracing::error!("Query_bound_phoneresFailed (GUID: {}): {}", guid, err);
            return APIResult {
                code: 500,
                reqststus: "failed".to_string(),
                data: json!(null),
            };
        }
    };
    match query_bound_phoneres.json::<Value>().await {
        Ok(data) => {
            tracing::info!("Query_bound_phoneresSuccess: {:?}", data);
            APIResult {
                code: 0,
                reqststus: "success".to_string(),
                data,
            }

        }
        Err(err) => {
            tracing::error!("SerializeQuery_bound_phoneresFailed (GUID: {}): {}", guid, err);
            APIResult {
                code: 500,
                reqststus: "Failed".to_string(),
                data: json!({ "error": err.to_string() }),
            }
        }
    }
}
pub async fn verify_mbphone( version:String, sig:String, randstr:String, guid:String,mobile:String,area_code:String) ->APIResult<Value> {
    tracing::info!(version, sig, randstr, guid, mobile, area_code);
    let client = Client::new();
    let body = json!({
             "com": {
                "src": 1,
                "scene": 100908,
                "platform": 2,
                "version": version,
                "unlgn": {
                    "uin": 0,
                    "sig": sig,
                    "sigType": 1,
                "randstr": randstr,
                }
             },
            "mobile": mobile,
            "areaCode": area_code
    });
    let verifymbphoneres = match client.post("https://accounts.qq.com/login/limit/proxy/domain/qq110.qq.com/v3/verifymbphone?uin=0&bkn=")
        .json(&body)
        .header("qname-service", "1935233:65536")
        .header("qname-space", "Production")
        .header("Cookie", "uin=; p_uin=; p_uid=; login_key_set_failed=AlreadyLogout")
        .send()
        .await
    {
        Ok(response) => {
            response
        },
        Err(err) => {
            tracing::error!("VerifymbphoneresFailed (GUID: {}): {}", guid, err);
            return APIResult {
                code: 500,
                reqststus: "failed".to_string(),
                data: json!(null),
            };
        }
    };
    match verifymbphoneres.json::<Value>().await {
        Ok(data) => {
            tracing::info!("VerifymbphoneresSuccess: {:?}", data);
            APIResult {
                code: 0,
                reqststus: "success".to_string(),
                data,
            }

        }
        Err(err) => {
            tracing::error!("SerializeVerifymbphoneresFailed (GUID: {}): {}", guid, err);
            APIResult {
                code: 500,
                reqststus: "Failed".to_string(),
                data: json!({ "error": err.to_string() }),
            }
        }
    }
}
pub async fn get_sms( version:String, sig:String, randstr:String, guid:String,area_code:String,phone_num:String) ->APIResult<Value> {
    tracing::info!(version, sig, randstr, guid, area_code, phone_num);
    let client = Client::new();
    let body = json!({
                "com": {
                    "src": 1,
                    "scene": 100331,
                    "platform": 2,
                    "version": version,
                    "unlgn": {
                        "uin": 0,
                        "sig": sig,
                        "sigType": 1,
                        "randstr": randstr,
                    }
              },
                "way": 4,
                "mobile": phone_num,
                "areaCode": area_code
    });
    let getsmsres = match client.post("https://accounts.qq.com/login/limit/proxy/domain/qq110.qq.com/v3/getsms?uin=0&bkn=")
        .json(&body)
        .header("qname-service", "1935233:65536")
        .header("qname-space", "Production")
        .header("Cookie", "uin=; p_uin=; p_uid=; login_key_set_failed=AlreadyLogout")
        .send()
        .await
    {
        Ok(response) => {
            response
        },
        Err(err) => {
            tracing::error!("GetsmsresFailed (GUID: {}): {}", guid, err);
            return APIResult {
                code: 500,
                reqststus: "failed".to_string(),
                data: json!(null),
            };
        }
    };
    match getsmsres.json::<Value>().await {
        Ok(data) => {
            tracing::info!("GetsmsresSuccess: {:?}", data);
            APIResult {
                code: 0,
                reqststus: "success".to_string(),
                data,
            }

        }
        Err(err) => {
            tracing::error!("SerializeGetsmsresFailed (GUID: {}): {}", guid, err);
            APIResult {
                code: 500,
                reqststus: "Failed".to_string(),
                data: json!({ "error": err.to_string() }),
            }
        }
    }
}
pub async fn chk_sms( version:String, sig:String, randstr:String, guid:String,phone_num:String,area_code:String) ->APIResult<Value> {
    tracing::info!(version, sig, randstr, guid, area_code);
    let client = Client::new();
    let body = json!({
             "com": {
                "src": 1,
                "scene": 100331,
                "platform": 2,
                "version": version,
                "unlgn": {
                    "uin": 0,
                    "sig": sig,
                    "sigType": 1,
                    "randstr": randstr,
                }
            },
            "way": 4,
            "mobile": phone_num,
            "areaCode": area_code
    });
    let chksmsres = match client.post("https://accounts.qq.com/login/limit/proxy/domain/qq110.qq.com/v3/chksms?uin=0&bkn=")
        .json(&body)
        .header("qname-service", "1935233:65536")
        .header("qname-space", "Production")
        .header("Cookie", "uin=; p_uin=; p_uid=; login_key_set_failed=AlreadyLogout")
        .send()
        .await
    {
        Ok(response) => {
            response
        },
        Err(err) => {
            tracing::error!("ChksmsresFailed (GUID: {}): {}", guid, err);
            return APIResult {
                code: 500,
                reqststus: "failed".to_string(),
                data: json!(null),
            };
        }
    };
    match chksmsres.json::<Value>().await {
        Ok(data) => {
            tracing::info!("ChksmsresSuccess: {:?}", data);
            APIResult {
                code: 0,
                reqststus: "success".to_string(),
                data,
            }

        }
        Err(err) => {
            tracing::error!("SerializeChksmsresFailed (GUID: {}): {}", guid, err);
            APIResult {
                code: 500,
                reqststus: "Failed".to_string(),
                data: json!({ "error": err.to_string() }),
            }
        }
    }
}
pub async fn auth_diff_password( version:String, sig:String, randstr:String, guid:String,qimei:String,phone_num:String,appid:String,key:Value) ->APIResult<Value> {
    tracing::info!(version, sig, randstr, guid, phone_num, appid);
    let client = Client::new();
    let body = json!({
              "com": {
    "src": 1,
    "scene": 103301,
    "platform": 2,
    "version": version,
    "unlgn": {
        "uin": 0,
        "sig": sig,
        "sigType": 1,
        "randstr": randstr,
    },
    "device": {
        "guid": guid,
        "qimei": qimei,
        "qimei36": qimei,
        "subappid": appid,
        "platform": "Android",
        "brand": "March",
        "model": "7"
    }
},
"token": sig,
"type": 0,
"ticket": {
    "ticket0": {
        "way": 4,
        "keyType": 40,
        "key": key,
        "mobile": phone_num
    }
    }
    });
    let auth_diff_passwordres = match client.post("https://accounts.qq.com/login/limit/proxy/domain/qq110.qq.com/v3/auth_diff_password?uin=0&bkn=")
        .json(&body)
        .header("qname-service", "1935233:65536")
        .header("qname-space", "Production")
        .header("Cookie", "uin=; p_uin=; p_uid=; login_key_set_failed=AlreadyLogout")
        .send()
        .await
    {
        Ok(response) => {
            response
        },
        Err(err) => {
            tracing::error!("Auth_diff_passwordFailed (GUID: {}): {}", guid, err);
            return APIResult {
                code: 500,
                reqststus: "failed".to_string(),
                data: json!(null),
            };
        }
    };
    match auth_diff_passwordres.json::<Value>().await {
        Ok(data) => {
            tracing::info!("Auth_diff_passwordSuccess: {:?}", data);
            APIResult {
                code: 0,
                reqststus: "success".to_string(),
                data,
            }

        }
        Err(err) => {
            tracing::error!("SerializeAuth_diff_passwordFailed (GUID: {}): {}", guid, err);
            APIResult {
                code: 500,
                reqststus: "Failed".to_string(),
                data: json!({ "error": err.to_string() }),
            }
        }
    }
}


pub async fn login_verify1(connect_info: ConnectInfo<SocketAddr>,Json(input):Json<Value>)->APIResult<Value>{
    tracing::info!("IP：{} 开始处理第一步验证",connect_info.0);

    let param1 = match &input {
        Object(map)=>   map,
        _ => {
            return APIResult{
                code:400,
                reqststus:"Must Json Params".to_string(),
                data:json!(null)
            };
        }
    };
    let version = param1.get("version").and_then(|t| t.as_str()).unwrap_or("9.2.5");
    let sig = param1.get("sig").and_then(|t| t.as_str()).unwrap_or("");
    let ticket = param1.get("ticket").and_then(|t| t.as_str()).unwrap_or("");
    let randstr = param1.get("randstr").and_then(|t| t.as_str()).unwrap_or("");
    let guid = param1.get("guid").and_then(|t| t.as_str()).unwrap_or("");

    let chkcaptchares = chk_captcha(version.to_string(),sig.to_string(),ticket.to_string(),randstr.to_string(),guid.to_string()).await;
    if chkcaptchares.data.get("retcode").and_then(|t1| {t1.as_i64()}).unwrap_or(-1)!= 0 {
        return chkcaptchares
    }

    let appid = param1.get("appid").and_then(|t2| {t2.as_str()}).unwrap_or("537306651");
    let chkriskres = chk_risk(version.to_string(),sig.to_string(),ticket.to_string(),randstr.to_string(),guid.to_string(),appid.to_string()).await;
    if chkriskres.data.get("retcode").and_then(|t1| {t1.as_i64()}).unwrap_or(-1)!= 0 {
        return chkriskres
    }
    let queryloginverifymethodres = query_login_verify_method(version.to_string(),sig.to_string(),randstr.to_string(),guid.to_string()).await;
    if queryloginverifymethodres.data.get("retcode").and_then(|t1| {t1.as_i64()}).unwrap_or(-1)!= 0 {
        return queryloginverifymethodres
    }
     query_bound_phone (version.to_string(),sig.to_string(),randstr.to_string(),guid.to_string()).await
}

pub async fn login_verify2(connect_info: ConnectInfo<SocketAddr>,Json(input):Json<Value>)->APIResult<Value>{
    tracing::info!("IP：{} 开始处理第二步验证",connect_info.0);

    let param1 = match &input {
        Object(map)=>   map,
        _=>{
            return APIResult{
                code:400,
                reqststus:"Must Json Params".to_string(),
                data:json!(null)
            };
        }
    };
    let version = param1.get("version").and_then(|t| t.as_str()).unwrap_or("9.2.5");
    let sig = param1.get("sig").and_then(|t| t.as_str()).unwrap_or("");
    let randomstr = param1.get("randstr").and_then(|t| t.as_str()).unwrap_or("");
    let guid = param1.get("guid").and_then(|t| t.as_str()).unwrap_or("");
    let area_code = param1.get("areaCode").and_then(|t| t.as_str()).unwrap_or("");
    let mobile = param1.get("mobile").and_then(|t| t.as_str()).unwrap_or("");
    let phone_num = param1.get("phoneNum").and_then(|t| t.as_str()).unwrap_or("");
    let verifymbphoneres = verify_mbphone(version.to_string(),sig.to_string(),randomstr.to_string(),guid.to_string(),mobile.to_string(),area_code.to_string()).await;
    if verifymbphoneres.data.get("retcode").and_then(|t1| {t1.as_i64()}).unwrap_or(-1) != 0 {
        return verifymbphoneres
    };
    let  get_sms_res = get_sms(version.to_string(),sig.to_string(),randomstr.to_string(),guid.to_string(),area_code.to_string(),phone_num.to_string()).await;
    get_sms_res
}
pub async fn login_verify3(connect_info: ConnectInfo<SocketAddr>,Json(input):Json<Value>)->APIResult<Value>{
    tracing::info!("IP：{} 开始处理第三步验证",connect_info.0);

    let param1 = match &input {
        Object(map)=>   map,
        _=>{
            return APIResult{
                code:400,
                reqststus:"Must Json Params".to_string(),
                data:json!(null)
            };
        }
    };
    let version = param1.get("version").and_then(|t| t.as_str()).unwrap_or("9.2.5");
    let sig = param1.get("sig").and_then(|t| t.as_str()).unwrap_or("");
    let randomstr = param1.get("randstr").and_then(|t| t.as_str()).unwrap_or("");
    let guid = param1.get("guid").and_then(|t| t.as_str()).unwrap_or("");
    let area_code = param1.get("areaCode").and_then(|t| t.as_str()).unwrap_or("");
    let appid = param1.get("appid").and_then(|t| t.as_str()).unwrap_or("");
    let phone_num = param1.get("phoneNum").and_then(|t| t.as_str()).unwrap_or("");
    let qimei=param1.get("qimei").and_then(|t| t.as_str()).unwrap_or("");
    let chk_smsres = chk_sms(version.to_string(),sig.to_string(),randomstr.to_string(),guid.to_string(),phone_num.to_string(),area_code.to_string()).await;
    if chk_smsres.data.get("retcode").and_then(|t1| {t1.as_i64()}).unwrap_or(-1) != 0 {
        return chk_smsres
    };
    let key = chk_smsres.data;
    let  auth_diff_password_res = auth_diff_password(version.to_string(),sig.to_string(),randomstr.to_string(),guid.to_string(),qimei.to_string(),phone_num.to_string(),appid.to_string(),key).await;
    auth_diff_password_res
}