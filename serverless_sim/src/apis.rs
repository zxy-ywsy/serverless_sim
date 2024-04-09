
use serde_json::json;
use serde::{Serialize, Deserialize};
use axum::{http::StatusCode, routing::post, Json, Router};
use async_trait::async_trait;
use crate::network::ApiHandlerImpl;


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTopoResp{
    Exist{
        topo:Vec<Vec<f64>>,
},
    NotFound{
        msg:String,
},

}

impl GetTopoResp {
    fn id(&self)->u32 {
        match self {
                GetTopoResp::Exist{..}=>1,
    GetTopoResp::NotFound{..}=>2,

        }
    }
    pub fn serialize(&self)->String {
        json!({
            "id": self.id(),
            "kernel": serde_json::to_value(self).unwrap(),
        }).to_string()
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetTopoReq {
        env_id:String,
}


#[async_trait]
pub trait ApiHandler {
    
    async fn handle_get_topo(&self, req:GetTopoReq)->GetTopoResp;
            
}


pub fn add_routers(mut router:Router)->Router
{
    
    async fn get_topo(Json(req):Json<GetTopoReq>)-> (StatusCode, Json<GetTopoResp>){
        (StatusCode::OK, Json(ApiHandlerImpl.handle_get_topo(req).await))
    }
    router=router
        .route("/get_topo", post(get_topo));
                             
    
    router
}

