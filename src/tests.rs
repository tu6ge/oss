
use std::env;
use super::*;
use dotenv::dotenv;


#[test]
fn test_get_bucket_list(){
  dotenv().ok();

  let key_id      = env::var("ALIYUN_KEY_ID").unwrap();
  let key_secret  = env::var("ALIYUN_KEY_SECRET").unwrap();
  let endpoint    = env::var("ALIYUN_ENDPOINT").unwrap();
  let bucket      = env::var("ALIYUN_BUCKET").unwrap();

  let client = client(&key_id,&key_secret, &endpoint, &bucket);

  let bucket_list = client.get_bucket_list();

  assert_matches!(bucket_list, Ok(_));
}

#[test]
fn test_get_bucket_info(){
  dotenv().ok();

  let key_id      = env::var("ALIYUN_KEY_ID").unwrap();
  let key_secret  = env::var("ALIYUN_KEY_SECRET").unwrap();
  let endpoint    = env::var("ALIYUN_ENDPOINT").unwrap();
  let bucket      = env::var("ALIYUN_BUCKET").unwrap();

  let client = client(&key_id,&key_secret, &endpoint, &bucket);

  let bucket_list = client.get_bucket_info();

  assert_matches!(bucket_list, Ok(_));
}


#[test]
fn test_get_object() {
  dotenv().ok();

  let key_id      = env::var("ALIYUN_KEY_ID").unwrap();
  let key_secret  = env::var("ALIYUN_KEY_SECRET").unwrap();
  let endpoint    = env::var("ALIYUN_ENDPOINT").unwrap();
  let bucket      = env::var("ALIYUN_BUCKET").unwrap();

  let client = client(&key_id,&key_secret, &endpoint, &bucket);

  let object_list = client.get_object_list();

  assert_matches!(object_list, Ok(_));
}

#[test]
fn test_put_and_delete_file(){
  dotenv().ok();

  let key_id      = env::var("ALIYUN_KEY_ID").unwrap();
  let key_secret  = env::var("ALIYUN_KEY_SECRET").unwrap();
  let endpoint    = env::var("ALIYUN_ENDPOINT").unwrap();
  let bucket      = env::var("ALIYUN_BUCKET").unwrap();

  let client = client(&key_id,&key_secret, &endpoint, &bucket);

  let object_list = client.put_file("examples/bg2015071010.png", "examples/bg2015071010.png");

  assert_matches!(object_list, Ok(_));

  let result = client.delete_object("examples/bg2015071010.png");

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