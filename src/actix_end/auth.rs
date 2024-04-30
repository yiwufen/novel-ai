#[cfg(feature = "ssr")]
use actix_web::web;
#[cfg(feature="ssr")]
use bcrypt::verify;
use bson::{doc, DateTime};
#[cfg(feature = "ssr")]
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use leptos::{logging::log, *};
#[cfg(feature = "ssr")]
use leptos_actix::extract;
#[cfg(feature = "ssr")]
use mongodb::Database;

use crate::app_models::user::{Claims, User};

#[server(Register, "/api")]
pub async fn register_api(
    email: String,
    password: String,
    name: String,
) -> Result<String, ServerFnError> {
    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| -> ServerFnError { ServerFnError::ServerError(e.to_string()) })?;

    let user = User {
        name: name,
        email: email,
        password: hashed_password,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    let db_client = extract::<web::Data<Database>>().await?.into_inner();
    let collection = db_client.collection::<User>("users");
    collection.insert_one(user, None).await?;
    leptos_actix::redirect("/");
    Ok("".to_string())
}

#[server(GetToken, "/api")]
pub async fn get_token_api(email: String, password: String) -> Result<String, ServerFnError> {
    let db_client = extract::<web::Data<Database>>().await?.into_inner();
    let collection = db_client.collection::<User>("users");
    let user = collection
        .find_one(doc! {"email": email}, None)
        .await?;
    log!("user: {:?}", user);
    match user {
        Some(user) => {
            match verify(password, &user.password) {
                Ok(_) => {
                    log!("password match");
                    let my_claims = Claims {
                        sub: "1234567890".to_owned(),
                        ..Default::default()
                    };
                    let secert_key = std::env::var("JWT_SECRET_KEY").map_err(|_| -> ServerFnError {
                        ServerFnError::ServerError("JWT_SECRET_KEY not found".to_string())
                    })?;
                    let encoding_key = EncodingKey::from_secret(secert_key.as_ref());
                    encode(&Header::default(), &my_claims, &encoding_key)
                        .map_err(|e| -> ServerFnError { ServerFnError::ServerError(e.to_string()) })
                }
                Err(_) => {
                    return Err(ServerFnError::ServerError("Password not match".to_string()));
                }
            }
            
        }
        None => {
            return Err(ServerFnError::ServerError("User not found".to_string()));
        }
    }
}

#[cfg(feature="ssr")]
pub fn auth_jwt(token: String) -> bool {
    let secert_key = std::env::var("JWT_SECRET_KEY").expect("[panck!!!!!!!!!!!!!!!!!! JWT_SECRET_KEY cannot get #################]");
    let encoding_key = DecodingKey::from_secret(secert_key.as_ref());
    // 设置 JWT 验证参数
    let mut validation = Validation::default();
    validation.leeway = 60;
    validation.validate_exp = true;
    validation.algorithms = vec![Algorithm::HS256];
    let token_data =
        jsonwebtoken::decode::<Claims>(&token, &encoding_key, &validation);
    match token_data {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use bson::doc;
    use mongodb::Client;

    use crate::app_models::user::User;


    #[actix_web::test]
    async fn test_auth_hash() {
        let email="199@qq.com".to_string();
        let password = "12345678".to_string();
        let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
        println!("{:?}", hashed_password);
        let mongodb_uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

        let mongo_client = Client::with_uri_str(mongodb_uri).await.expect("failed to connect");
        let db = mongo_client.database("novel");
        let collection = db.collection::<User>("users");
        let res = collection
            .find_one(doc! {"email": email, "password": hashed_password}, None)
            .await
            .expect("failed to find");
        println!("{:#?}", res);
        
    }
}