
use std::{env, collections::HashMap, path::PathBuf};

use dotenv::dotenv;

use assert_matches::assert_matches;

#[tokio::test]
async fn test_get_bucket_list(){
  dotenv().ok();

  let key_id      = env::var("ALIYUN_KEY_ID").unwrap();
  let key_secret  = env::var("ALIYUN_KEY_SECRET").unwrap();
  let endpoint    = env::var("ALIYUN_ENDPOINT").unwrap();
  let bucket      = env::var("ALIYUN_BUCKET").unwrap();

  let client = crate::client(&key_id,&key_secret, &endpoint, &bucket);

  let bucket_list = client.get_bucket_list().await;

  assert_matches!(bucket_list, Ok(_));
}

#[tokio::test]
async fn test_get_bucket_info(){
  dotenv().ok();

  let key_id      = env::var("ALIYUN_KEY_ID").unwrap();
  let key_secret  = env::var("ALIYUN_KEY_SECRET").unwrap();
  let endpoint    = env::var("ALIYUN_ENDPOINT").unwrap();
  let bucket      = env::var("ALIYUN_BUCKET").unwrap();

  let client = crate::client(&key_id,&key_secret, &endpoint, &bucket);

  let bucket_list = client.get_bucket_info().await;

  assert_matches!(bucket_list, Ok(_));
}

#[tokio::test]
async fn get_object_by_bucket_struct(){
  dotenv().ok();

  let key_id      = env::var("ALIYUN_KEY_ID").unwrap();
  let key_secret  = env::var("ALIYUN_KEY_SECRET").unwrap();
  let endpoint    = env::var("ALIYUN_ENDPOINT").unwrap();

  let client = crate::client(&key_id,&key_secret, &endpoint, "");

  let bucket_list = client.get_bucket_list().await.unwrap();
  let mut query:HashMap<String,String> = HashMap::new();
  query.insert("max-keys".to_string(), "5".to_string());
  query.insert("prefix".to_string(), "babel".to_string());

  let buckets = bucket_list.buckets;
  let the_bucket = &buckets[0];
  let object_list = the_bucket.get_object_list(query).await;
  assert_matches!(object_list, Ok(_));
}

#[tokio::test]
async fn test_get_object() {
  dotenv().ok();

  let key_id      = env::var("ALIYUN_KEY_ID").unwrap();
  let key_secret  = env::var("ALIYUN_KEY_SECRET").unwrap();
  let endpoint    = env::var("ALIYUN_ENDPOINT").unwrap();
  let bucket      = env::var("ALIYUN_BUCKET").unwrap();

  let client = crate::client(&key_id,&key_secret, &endpoint, &bucket);
  let query: HashMap<String,String> = HashMap::new();

  let object_list = client.get_object_list(query).await;

  assert_matches!(object_list, Ok(_));
}

#[tokio::test]
async fn test_put_and_delete_file(){
  dotenv().ok();

  let key_id      = env::var("ALIYUN_KEY_ID").unwrap();
  let key_secret  = env::var("ALIYUN_KEY_SECRET").unwrap();
  let endpoint    = env::var("ALIYUN_ENDPOINT").unwrap();
  let bucket      = env::var("ALIYUN_BUCKET").unwrap();

  let client = crate::client(&key_id,&key_secret, &endpoint, &bucket);

  let object_list = client.put_file(PathBuf::from("examples/bg2015071010.png"), "examples/bg2015071010.png").await;

  assert_matches!(object_list, Ok(_));

  let result = client.delete_object("examples/bg2015071010.png").await;

  assert_matches!(result, Ok(_));
}

// #[bench]
// fn bench_get_object(b: &mut Bencher){
//   dotenv().ok();

//   let key_id      = env::var("ALIYUN_KEY_ID").unwrap();
//   let key_secret  = env::var("ALIYUN_KEY_SECRET").unwrap();
//   let endpoint    = env::var("ALIYUN_ENDPOINT").unwrap();
//   let bucket      = env::var("ALIYUN_BUCKET").unwrap();

//   let client = client::Client::new(&key_id,&key_secret, &endpoint, &bucket);
//   b.iter(|| {
//     client.get_object_list();
//   });
// }