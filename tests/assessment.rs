use eyre::Result;
use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::ThreadRng,
    thread_rng, Rng,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[tokio::test]
async fn mirrors_what_is_passed_in() -> Result<()> {
    let mut rng = thread_rng();
    let random_json = RandomData::new(&mut rng);
    let random_path_value = create_random_string(4, &mut rng).to_lowercase();
    let random_query_param = rng.gen::<i32>();
    let url = format!(
        "http://localhost:3000/{}?id={random_query_param}",
        &random_path_value
    );

    let client = Client::new();
    let response = client.post(url).json(&random_json).send().await?;
    let status = response.status();

    assert_eq!(status, 200);

    let response_data = response.json::<ResponseRandomData>().await?;

    assert_eq!(response_data.json, random_json);
    assert_eq!(response_data.path, random_path_value);
    assert_eq!(response_data.query.id, random_query_param);

    Ok(())
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct RandomData {
    username: String,
    password: String,
    favorite_number: i32,
}

impl RandomData {
    pub fn new(rng: &mut ThreadRng) -> Self {
        Self {
            username: create_random_string(8, rng),
            password: create_random_string(16, rng),
            favorite_number: rng.gen(),
        }
    }
}

fn create_random_string(length: usize, rng: &mut ThreadRng) -> String {
    Alphanumeric.sample_string(rng, length)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ResponseRandomData {
    pub json: RandomData,
    pub path: String,
    pub query: ResponseQuery,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ResponseQuery {
    pub id: i32,
}
