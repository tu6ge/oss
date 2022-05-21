
use crate::client::{Client, OssObject, Result};
use crate::auth::VERB;
use chrono::prelude::*;
use url::Url;

use quick_xml::{events::Event, Reader};

#[derive(Clone, Debug)]
pub struct ListBuckets {
    prefix: Option<String>,
    marker: Option<String>,
    max_keys: Option<String>,
    is_truncated: bool,
    next_marker: Option<String>,

    id: Option<String>,
    display_name: Option<String>,

    buckets: Vec<Bucket>,
}

impl ListBuckets {
  pub fn new(
    prefix: Option<String>, 
    marker: Option<String>,
    max_keys: Option<String>,
    is_truncated: bool,
    next_marker: Option<String>,
    id: Option<String>,
    display_name: Option<String>,
    buckets: Vec<Bucket>,
  ) -> ListBuckets {
    ListBuckets {
      prefix,
      marker,
      max_keys,
      is_truncated,
      next_marker,
      id,
      display_name,
      buckets
    }
  }
}

impl OssObject for ListBuckets  {

  fn from_xml(xml: String) -> Result<ListBuckets> {
    let mut result = Vec::new();
    let mut reader = Reader::from_str(xml.as_str());
    reader.trim_text(true);
    let mut buf = Vec::with_capacity(xml.len());
    let mut skip_buf = Vec::with_capacity(xml.len());

    let mut prefix = String::new();
    let mut marker = String::new();
    let mut max_keys = String::new();
    let mut is_truncated = false;
    let mut next_marker = String::new();
    let mut id = String::with_capacity(8);
    let mut display_name = String::with_capacity(8);

    let mut name = String::new();
    let mut location = String::new();
    let mut creation_date = String::with_capacity(20);
    
    // 目前最长的可用区 zhangjiakou 13 ，剩余部分总共 20 
    let mut extranet_endpoint = String::with_capacity(33);
    // 上一个长度 + 9 （-internal）
    let mut intranet_endpoint = String::with_capacity(42);
    // 最长的值 ColdArchive 11
    let mut storage_class = String::with_capacity(11);

    let list_buckets;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"Prefix" => prefix = reader.read_text(e.name(), &mut skip_buf)?,
                b"Marker" => marker = reader.read_text(e.name(), &mut skip_buf)?,
                b"MaxKeys" => max_keys = reader.read_text(e.name(), &mut skip_buf)?,
                b"IsTruncated" => {
                    is_truncated = reader.read_text(e.name(), &mut skip_buf)? == "true"
                }
                b"NextMarker" => next_marker = reader.read_text(e.name(), &mut skip_buf)?,
                b"ID" => id = reader.read_text(e.name(), &mut skip_buf)?,
                b"DisplayName" => display_name = reader.read_text(e.name(), &mut skip_buf)?,

                b"Bucket" => {
                    name.clear();
                    location.clear();
                    creation_date.clear();
                    extranet_endpoint.clear();
                    intranet_endpoint.clear();
                    storage_class.clear();
                }

                b"Name" => name = reader.read_text(e.name(), &mut skip_buf)?,
                b"CreationDate" => creation_date = reader.read_text(e.name(), &mut skip_buf)?,
                b"ExtranetEndpoint" => {
                    extranet_endpoint = reader.read_text(e.name(), &mut skip_buf)?
                }
                b"IntranetEndpoint" => {
                    intranet_endpoint = reader.read_text(e.name(), &mut skip_buf)?
                }
                b"Location" => location = reader.read_text(e.name(), &mut skip_buf)?,
                b"StorageClass" => {
                    storage_class = reader.read_text(e.name(), &mut skip_buf)?
                }
                _ => (),
            },
            Ok(Event::End(ref e)) if e.name() == b"Bucket" => {
              let in_creation_date = &creation_date.parse::<DateTime<Utc>>()?;
              let bucket = Bucket::new(
                  name.clone(),
                  in_creation_date.clone(),
                  location.clone(),
                  extranet_endpoint.clone(),
                  intranet_endpoint.clone(),
                  storage_class.clone(),
              );
              result.push(bucket);
            }
            Ok(Event::Eof) => {
                list_buckets = ListBuckets::new(
                    Client::string2option(prefix),
                    Client::string2option(marker),
                    Client::string2option(max_keys),
                    is_truncated,
                    Client::string2option(next_marker),
                    Client::string2option(id),
                    Client::string2option(display_name),
                    result,
                );
                break;
            } // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
        buf.clear();
    }
    Ok(list_buckets)
  }
}



#[derive(Clone, Debug)]
pub struct Bucket{
  // bucket_info: Option<Bucket<'b>>,
  // bucket: Option<Bucket<'c>>,
  pub creation_date: DateTime<Utc>,
  pub extranet_endpoint: String,
  pub intranet_endpoint: String,
  pub location: String,
  pub name: String,
  // owner 	存放Bucket拥有者信息的容器。父节点：BucketInfo.Bucket
  // access_control_list;
  // pub grant: Grant,
  // pub data_redundancy_type: Option<DataRedundancyType>,
  pub storage_class: String,
  // pub versioning: &'a str,
  // ServerSideEncryptionRule,
  // ApplyServerSideEncryptionByDefault,
  // pub sse_algorithm: &'a str,
  // pub kms_master_key_id: Option<&'a str>,
  // pub cross_region_replication: &'a str,
  // pub transfer_acceleration: &'a str,
}

impl Bucket {
  pub fn new(
    name: String,
    creation_date: DateTime<Utc>,
    location: String,
    extranet_endpoint: String,
    intranet_endpoint: String,
    storage_class: String
  ) -> Bucket {
    Bucket {
      name,
      creation_date,
      // data_redundancy_type: None,
      location,
      extranet_endpoint,
      intranet_endpoint,
      storage_class,
    }
  }
}

impl OssObject for Bucket {

  fn from_xml(xml: String) -> Result<Self>{
    let mut reader = Reader::from_str(xml.as_str());
    reader.trim_text(true);
    let mut buf = Vec::with_capacity(xml.len());
    let mut skip_buf = Vec::with_capacity(xml.len());

    let mut name = String::new();
    let mut location = String::new();
    let mut creation_date = String::with_capacity(20);
    
    // 目前最长的可用区 zhangjiakou 13 ，剩余部分总共 20 
    let mut extranet_endpoint = String::with_capacity(33);
    // 上一个长度 + 9 （-internal）
    let mut intranet_endpoint = String::with_capacity(42);
    // 最长的值 ColdArchive 11
    let mut storage_class = String::with_capacity(11);

    let bucket;

    loop {
      match reader.read_event(&mut buf) {
          Ok(Event::Start(ref e)) => match e.name() {
              b"Name" => name = reader.read_text(e.name(), &mut skip_buf)?,
              b"CreationDate" => creation_date = reader.read_text(e.name(), &mut skip_buf)?,
              b"ExtranetEndpoint" => {
                  extranet_endpoint = reader.read_text(e.name(), &mut skip_buf)?
              }
              b"IntranetEndpoint" => {
                  intranet_endpoint = reader.read_text(e.name(), &mut skip_buf)?
              }
              b"Location" => location = reader.read_text(e.name(), &mut skip_buf)?,
              b"StorageClass" => {
                  storage_class = reader.read_text(e.name(), &mut skip_buf)?
              }
              _ => (),
          },
          Ok(Event::Eof) => {
            let in_creation_date = &creation_date.parse::<DateTime<Utc>>()?;
            bucket = Bucket::new(
              name.clone(),
              in_creation_date.clone(),
              location.clone(),
              extranet_endpoint.clone(),
              intranet_endpoint.clone(),
              storage_class.clone(),
            );
            break;
          } // exits the loop when reaching end of file
          Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
          _ => (), // There are several other `Event`s we do not consider here
      }
      buf.clear();
    }
    Ok(bucket)
  }
}


impl<'a> Client<'a> {

  /** # 获取 buiket 列表
      # Examples1
```
use dotenv::dotenv;
use std::env;
use aliyun_oss_client::client;

let key_id      = env::var("ALIYUN_KEY_ID").unwrap();
let key_secret  = env::var("ALIYUN_KEY_SECRET").unwrap();
let endpoint    = env::var("ALIYUN_ENDPOINT").unwrap();
let bucket      = env::var("ALIYUN_BUCKET").unwrap();

let client = client::Client::new(&key_id,&key_secret, &endpoint, &bucket);

let response = client.get_bucket_list().unwrap();
let first = response.first().unwrap();
assert_eq!(first, "abc");
```

  */
  pub fn get_bucket_list(&self) -> Result<ListBuckets> {
    let url = Url::parse(&self.endpoint).unwrap();

    let response = self.builder(VERB::GET, &url, None);
    //println!("get_bucket_list {}", response.send().unwrap().text().unwrap());
    let content = response.send().expect(Client::ERROR_REQUEST_ALIYUN_API);

    Client::handle_error(&content);

    ListBuckets::from_xml(content.text().unwrap())
  }

  pub fn get_bucket_info(&self) -> Result<Bucket> {
    let headers = None;
    let mut bucket_url = self.get_bucket_url().unwrap();
    bucket_url.set_query(Some("bucketInfo"));

    let response = self.builder(VERB::GET, &bucket_url, headers);
    //println!("get_bucket_list {}", response.send().unwrap().text().unwrap());
    let content = response.send().expect(Client::ERROR_REQUEST_ALIYUN_API);

    Client::handle_error(&content);

    Bucket::from_xml(content.text().unwrap())
  }
}

pub enum Grant{
  Private,
  PublicRead,
  PublicReadWrite,
}

impl Default for Grant {
  fn default() -> Self {
    Self::Private
  }
}

#[derive(Clone, Debug)]
pub enum DataRedundancyType{
  LRS,
  ZRS,
}

impl Default for DataRedundancyType{
  fn default() -> Self {
    Self::LRS
  }
}


#[derive(Default,Clone, Debug)]
pub struct BucketListObjectParms<'a>{
  pub list_type: u8,
  pub delimiter: &'a str,
  pub continuation_token: &'a str,
  pub max_keys: u32,
  pub prefix: &'a str,
  pub encoding_type: &'a str,
  pub fetch_owner: bool,
}

#[derive(Default,Clone, Debug)]
pub struct BucketListObject<'a>{
  //pub content:
  pub common_prefixes: &'a str,
  pub delimiter: &'a str,
  pub encoding_type: &'a str,
  pub display_name: &'a str,
  pub etag: &'a str,
  pub id: &'a str,
  pub is_truncated: bool,
  pub key: &'a str,
  pub last_modified: &'a str, // TODO 时间
  pub list_bucket_result: Option<&'a str>,
  pub start_after: Option<&'a str>,
  pub max_keys: u32,
  pub name: &'a str,
  // pub owner: &'a str,
  pub prefix: &'a str,
  pub size: u64,
  pub storage_class: &'a str,
  pub continuation_token: Option<&'a str>,
  pub key_count: i32,
  pub next_continuation_token: Option<&'a str>,
  pub restore_info: Option<&'a str>,
}

#[derive(Clone, Debug)]
pub enum Location {
  CnHangzhou,
  CnShanghai,
  CnQingdao,
  CnBeijing,
  CnZhangjiakou, // 张家口 lenght=13
  CnHongkong,
  CnShenzhen,
  UsWest1,
  UsEast1,
  ApSouthEast1,
}

#[derive(Clone, Debug)]
pub struct BucketStat{
  pub storage: u64,
  pub object_count: u32,
  pub multipart_upload_count: u32,
  pub live_channel_count: u32,
  pub last_modified_time: u16,
  pub standard_storage: u64,
  pub standard_object_count: u32,
  pub infrequent_access_storage: u64,
  pub infrequent_access_real_storage: u64,
  pub infrequent_access_object_count: u64,
  pub archive_storage: u64,
  pub archive_real_storage: u64,
  pub archive_object_count: u64,
  pub cold_archive_storage: u64,
  pub cold_archive_real_storage: u64,
  pub cold_archive_object_count: u64,
}