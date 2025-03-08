use anyhow::Result;
use chrono::{DateTime, Days, Utc};
use fake::{
    faker::{chrono::en::DateTimeBetween, internet::en::SafeEmail, name::zh_cn::Name}, rand, Dummy, Fake, Faker
};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Dummy, Serialize, Deserialize)]
struct UserStat {
    #[dummy(faker = "UniqueEmail")]
    email: String,
    #[dummy(faker = "Name()")]
    name: String,
    #[dummy(faker = "DateTimeBetween(start(365*5), end())")]
    created_at: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(start(30), end())")]
    last_visited_at: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(start(90), end())")]
    last_watched_at: DateTime<Utc>,
    #[dummy(faker = "IntList(50, 100000, 100000)")]
    recent_watched: Vec<i32>,
    #[dummy(faker = "IntList(50, 200000, 100000)")]
    viewed_but_not_started: Vec<i32>,
    #[dummy(faker = "IntList(50, 300000, 100000)")]
    started_but_not_finished: Vec<i32>,
    #[dummy(faker = "IntList(50, 400000, 100000)")]
    finished: Vec<i32>,
    #[dummy(faker = "DateTimeBetween(start(45), end())")]
    last_email_notificatiion: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(start(15), end())")]
    last_in_app_notification: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(start(90), end())")]
    last_sms_notification: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let user: UserStat = Faker.fake();
    println!("{:#?}", user);

    Ok(())
}

fn start(days: u64) -> DateTime<Utc> {
    DateTime::from(Utc::now())
        .checked_sub_days(Days::new(days))
        .unwrap()
}

fn end() -> DateTime<Utc> {
    let now = Utc::now();
    DateTime::from(now)
}

struct IntList(pub i32, pub i32, pub i32);

impl Dummy<IntList> for Vec<i32> {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(v: &IntList, rng: &mut R) -> Vec<i32> {
        let (max, start, len) = (v.0, v.1, v.2);
        let size = rng.random_range(0..max);
        (0..size)
            .map(|_| {
                let start = start + rng.random::<i32>() % len;
                start
            })
            .collect()

    }
}

struct UniqueEmail;
const ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];
impl Dummy<UniqueEmail> for String {
    fn dummy_with_rng<R: rand::Rng +?Sized>(_: &UniqueEmail, rng: &mut R) -> String {
        let email: String = SafeEmail().fake_with_rng(rng);
        let id = nanoid!(8, &ALPHABET);
        let at = email.find('@').unwrap();
        format!("{}.{}@{}", &email[..at], id, &email[at+1..])
    }
}