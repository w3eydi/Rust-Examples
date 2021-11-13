#[macro_use] extern crate rocket;

use rocket::local::blocking::Client;

// Test that manual/auto ranking works as expected.

#[get("/<_number>")]
fn get0(_number: u8) -> &'static str { "0" }

#[get("/<_number>", rank = 1)]
fn get1(_number: u16) -> &'static str { "1" }

#[get("/<_number>", rank = 2)]
fn get2(_number: u32) -> &'static str { "2" }

#[get("/<_number>", rank = 3)]
fn get3(_number: u64) -> &'static str { "3" }

#[test]
fn test_ranking() {
    let rocket = rocket::build().mount("/", routes![get0, get1, get2, get3]);
    let client = Client::debug(rocket).unwrap();

    let response = client.get("/0").dispatch();
    assert_eq!(response.into_string().unwrap(), "0");

    let response = client.get(format!("/{}", 1 << 8)).dispatch();
    assert_eq!(response.into_string().unwrap(), "1");

    let response = client.get(format!("/{}", 1 << 16)).dispatch();
    assert_eq!(response.into_string().unwrap(), "2");

    let response = client.get(format!("/{}", 1u64 << 32)).dispatch();
    assert_eq!(response.into_string().unwrap(), "3");
}

// Test a collision due to same auto rank.

#[get("/<_n>")]
fn get0b(_n: u8) {  }

#[test]
fn test_rank_collision() {
    use rocket::error::ErrorKind;

    let rocket = rocket::build().mount("/", routes![get0, get0b]);
    let client_result = Client::debug(rocket);
    match client_result.as_ref().map_err(|e| e.kind()) {
        Err(ErrorKind::Collisions(..)) => { /* o.k. */ },
        Ok(_) => panic!("client succeeded unexpectedly"),
        Err(e) => panic!("expected collision, got {}", e)
    }
}
