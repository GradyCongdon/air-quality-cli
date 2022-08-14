struct Station {
    name: String,
    aqi: String,
    lat: f64,
    long: f64,
}

fn main() {
    let api_token = std::env::var("API_TOKEN") .expect("you need to have an API_TOKEN");
    
        let mut arg_iterator = std::env::args();
        arg_iterator.next();
        let args: String = arg_iterator.collect();

    
    let client = reqwest::blocking::Client::new();

    let response = client
        .get("https://api.waqi.info/search/")
        .query(&[("token", api_token), ("keyword", args)])
        .header(reqwest::header::USER_AGENT, "rusty air")
        .send()
        .expect("two hundo")
        .json::<serde_json::Value>()
        .expect("body by json");


    let data = response.get("data").unwrap().as_array().unwrap();

    let mut mine = Vec::new();

    for s in data.iter() {
        let aqi = s.get("aqi").unwrap().as_str().unwrap();

        let station = s.pointer("/station/name").unwrap().as_str().unwrap();
        let lat = s.pointer("/station/geo/0").unwrap().as_f64().unwrap();
        let long = s.pointer("/station/geo/1").unwrap().as_f64().unwrap();
        let station = Station {
            aqi: String::from(aqi),
            name: String::from(station),
            lat: lat,
            long: long,
        };
        if aqi != "-" {
            mine.push(station);
        }
    }

    mine.sort_by(|a, b| {
        a.long.partial_cmp(&b.long).unwrap()
    });

    for s in &mine {
        println!("{} - {} ({}, {})", s.aqi, s.name, s.lat, s.long);
    }

    let aqis: Vec<f64> = mine.iter().map(|m| m.aqi.parse::<f64>().unwrap()).collect();
    let min = *aqis.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max = *aqis.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    let sparky = sparkline::select_sparkline(sparkline::SparkThemeName::Colour);
    for aqi in aqis.iter() {
        let s : &String = sparky.spark(min, max, *aqi);
        print!("{} ", s);
    }
    println!("")
    

}
