
use std::collections::HashMap;
use std::fmt;

use crate::client::{Client, ReqeustHandler};
use crate::auth::VERB;
use crate::errors::{OssResult,OssError};
use crate::object::ObjectList;
use crate::traits::{ObjectListTrait, BucketTrait, ListBucketTrait};
use chrono::prelude::*;
use reqwest::Url;

#[derive(Clone)]
pub struct ListBuckets<'a> {
  pub prefix: Option<String>,
  pub marker: Option<String>,
  pub max_keys: Option<String>,
  pub is_truncated: bool,
  pub next_marker: Option<String>,
  pub id: Option<String>,
  pub display_name: Option<String>,
  pub buckets: Vec<Bucket<'a>>,
  client: Option<&'a Client<'a>>,
}

impl fmt::Debug for ListBuckets<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    f.debug_struct("ListBuckets")
      .field("prefix", &self.prefix)
      .field("marker", &self.marker)
      .field("max_keys", &self.max_keys)
      .field("is_truncated", &self.is_truncated)
      .field("next_marker", &self.next_marker)
      .field("id", &self.id)
      .field("display_name", &self.display_name)
      .field("buckets", &"bucket list")
      .finish()
  }
}

impl ListBucketTrait for ListBuckets<'_> {
  type Bucket = Bucket<'static>;
  fn from_oss(
    prefix: Option<String>, 
    marker: Option<String>,
    max_keys: Option<String>,
    is_truncated: bool,
    next_marker: Option<String>,
    id: Option<String>,
    display_name: Option<String>,
    buckets: Vec<Bucket>,
  ) -> OssResult<ListBuckets<'_>> {
    Ok(ListBuckets {
      prefix,
      marker,
      max_keys,
      is_truncated,
      next_marker,
      id,
      display_name,
      buckets,
      client: None,
    })
  }
}

impl <'b> ListBuckets<'b>  {
  pub fn set_client(&mut self, client: &'b Client){
    self.client = Some(client);
    for i in self.buckets.iter_mut() {
      i.set_client(client);
    }
  }
}



#[derive(Clone)]
pub struct Bucket<'a>{
  // bucket_info: Option<Bucket<'b>>,
  // bucket: Option<Bucket<'c>>,
  pub creation_date: DateTime<Utc>,
  pub extranet_endpoint: String,
  pub intranet_endpoint: String,
  pub location: String,
  pub name: String,
  // owner 	??????Bucket???????????????????????????????????????BucketInfo.Bucket
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
  client: Option<&'a Client<'a>>,
}

impl fmt::Debug for Bucket<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Bucket")
      .field("creation_date", &self.creation_date)
      .field("extranet_endpoint", &self.extranet_endpoint)
      .field("intranet_endpoint", &self.intranet_endpoint)
      .field("location", &self.location)
      .field("name", &self.name)
      .field("storage_class", &self.storage_class)
      .finish()
  }
}

impl BucketTrait for Bucket<'_> {
  fn from_oss(
    name: String,
    creation_date: String,
    location: String,
    extranet_endpoint: String,
    intranet_endpoint: String,
    storage_class: String,
  ) -> OssResult<Bucket<'static>> {
    Ok(Bucket {
      name,
      creation_date: creation_date.parse::<DateTime<Utc>>()?,
      // data_redundancy_type: None,
      location,
      extranet_endpoint,
      intranet_endpoint,
      storage_class,
      client: None,
    })
  }
}
impl <'b> Bucket<'b> {
  pub fn set_client(&mut self, client: &'b Client){
    self.client = Some(client);
  }

  pub fn client(&self) -> &Client{
    self.client.unwrap()
  }

  #[cfg(feature = "blocking")]
  pub fn blocking_get_object_list(&self, query: HashMap<String, String>) -> OssResult<ObjectList>{
    let input = "https://".to_owned() + &self.name + "." + &self.extranet_endpoint;
    let mut url = Url::parse(&input).map_err(|_| OssError::Input("url parse error".to_string()))?;

    let query_str = Client::<'b>::object_list_query_generator(&query);

    url.set_query(Some(&query_str));

    let client  = self.client.unwrap();

    let response = client.blocking_builder(VERB::GET, &url, None, Some(self.name.to_string()))?;
    let content = response.send()?.handle_error()?;
    Ok(
      ObjectList::from_xml(content.text()?)?.set_client(&client).set_search_query(query)
    )
  }

  pub async fn get_object_list(&self, query: HashMap<String, String>) -> OssResult<ObjectList<'_>>{
    let input = "https://".to_owned() + &self.name + "." + &self.extranet_endpoint;
    let mut url = Url::parse(&input).map_err(|_| OssError::Input("url parse error".to_string()))?;

    let query_str = Client::<'b>::object_list_query_generator(&query);

    url.set_query(Some(&query_str));

    let response = self.client.unwrap().builder(VERB::GET, &url, None, Some(self.name.to_string())).await?;
    let content = response.send().await?.handle_error()?;

    // println!("{}", &content.text()?);
    // return Err(errors::OssError::Other(anyhow!("abc")));

    Ok(
      ObjectList::from_xml(content.text().await?)?.set_client(&self.client.unwrap()).set_search_query(query)
    )
  }
}


impl<'a> Client<'a> {

  /** # ?????? buiket ??????
  */
  #[cfg(feature = "blocking")]
  pub fn blocking_get_bucket_list(&self) -> OssResult<ListBuckets> {
    let url = Url::parse(&self.endpoint).map_err(|_| OssError::Input("endpoint url parse error".to_string()))?;
    //url.set_path(self.bucket)

    let response = self.blocking_builder(VERB::GET, &url, None, None)?;
    let content = response.send()?.handle_error()?;
    let mut list = ListBuckets::from_xml(content.text()?)?;
    list.set_client(&self);
    Ok(list)
  }

  pub async fn get_bucket_list(&self) -> OssResult<ListBuckets<'_>>{
    let url = Url::parse(&self.endpoint).map_err(|_| OssError::Input("endpoint url parse error".to_string()))?;
    //url.set_path(self.bucket)

    let response = self.builder(VERB::GET, &url, None, None).await?;
    let content = response.send().await?.handle_error()?;
    
    let mut list = ListBuckets::from_xml(content.text().await?)?;
    list.set_client(&self);
    Ok(list)
  }

  #[cfg(feature = "blocking")]
  pub fn blocking_get_bucket_info(&self) -> OssResult<Bucket> {
    let headers = None;
    let mut bucket_url = self.get_bucket_url()?;
    bucket_url.set_query(Some("bucketInfo"));

    let response = self.blocking_builder(VERB::GET, &bucket_url, headers, None)?;
    let content = response.send()?.handle_error()?;
    let mut bucket = Bucket::from_xml(content.text()?)?;
    bucket.set_client(&self);
    Ok(bucket)
  }

  pub async fn get_bucket_info(&self) -> OssResult<Bucket<'_>> {
    let headers = None;
    let mut bucket_url = self.get_bucket_url()?;
    bucket_url.set_query(Some("bucketInfo"));

    let response = self.builder(VERB::GET, &bucket_url, headers, None).await?;
    let content = response.send().await?.handle_error()?;

    let mut bucket = Bucket::from_xml(content.text().await?)?;
    bucket.set_client(&self);
    
    Ok(bucket)
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
  pub last_modified: &'a str, // TODO ??????
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
  CnZhangjiakou, // ????????? lenght=13
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