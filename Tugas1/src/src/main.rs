extern crate scraper;
extern crate reqwest;
extern crate serde_json;

mod block;

#[macro_use]
extern crate serde_derive;

use scraper::{Html, Selector};
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::PathBuf;
use std::time::Duration;
use std::thread;
use std::io;
use block::Block;

#[derive(Serialize, Deserialize)]
struct Result {
	results : Vec<Block>,
	length : usize
}

fn main() {
	// Initialize vector
	let mut vector_res : Vec<Block> = Vec::new();
	// Get the result
	let doc_text = get_text("https://etherscan.io/blocks");
	let _last_block = get_first_block(doc_text);
	// Check if not found
	if _last_block == "Not Found" {
		println!("Failed to parse document!");
		return;
	}

	let mut input_text = String::new();
	

	println!("How many datas you want to scrape?");
	io::stdin()
		.read_line(&mut input_text)
		.expect("Failed to read from stdin");

	let trimmed = input_text.trim();
	
	let mut total_data : i32 = trimmed.parse().unwrap();

	if total_data < 0 {
		total_data *= -1;
	}

	// Change to int
	let _last_block_int : i32 = _last_block.parse().unwrap();
	// Iterate 5k times to get block data
	// Well :(
	for x in 0 .. total_data { 
		let _block = get_block(x, _last_block_int - x);

		let mut pause = false;
		match _block {
			Some(block) => vector_res.push(block),
			None => pause = true,
		}

		if pause {
			println!("Failed to retrieve!");
			thread::sleep(Duration::from_millis(1000));
		}
	}
	// New Result!
	let result = Result {
		length : vector_res.len(),
		results : vector_res
	};
	// To JSON
	let json_res = serde_json::to_string(&result);
	let json_string = json_res.unwrap();
	// Write to file
	write_to_file(&json_string);
}

fn write_to_file(input : &String){
	// Reading file
	let path = env::current_dir().unwrap();
	let parent_path = path.parent().unwrap();
	let mut to_path = PathBuf::from(parent_path.display().to_string());
	// Change directory
	to_path.push("data");
	// Name of the file
	to_path.push("result.json");
	println!("The current directory is {}", to_path.display());
	let f = File::create(to_path.display().to_string());
    let mut out_file = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
	out_file.write(input.as_bytes()).expect("Could not write file!");
}

fn get_text(url : &str) -> String {
	// let client = reqwest::Client::builder()
	// 	.gzip(true)
	// 	.timeout(Duration::from_secs(180))
	// 	.build().unwrap();

	// Initialized
	let mut res_resp = None;	
	let mut retry = true;
	
	while retry {
		// Request From the block
		let resp = reqwest::get(url);//.send();
		match resp {
			Ok(resp) => res_resp = Some(resp),
			Err(_error) => println!("Error Mesagge : {:?}", _error)
		};

		if res_resp.is_some() {
			retry = false;
		} else {
			println!("Error getting response.. Will try it again..");
			thread::sleep(Duration::from_millis(1500));
		}
	}

	// Get Option
	let opt = res_resp.unwrap().text();
	// Get Result
	let res = opt.iter().next();
	// Get Text
	let doc_text = match res {
		Some(text) => text,
		None => "Nothing",
	};
	// Return
	doc_text.to_string()
}

fn get_first_block(doc_text : String) -> String {
	// Url
	let mut url:String = "https://etherscan.io".to_owned();
	// Parse the result
	let doc = Html::parse_document(&doc_text);
	// Get selector
	let selector_a = Selector::parse("a").unwrap();
	// Iterate
	for el_a in doc.select(&selector_a) {
		//println!("{}", el_a);
		let attrs = el_a.value().attrs();
		let value = el_a.inner_html();
		for (key, val) in attrs {
			if key == "href" && val.contains("/block/") {
				url.push_str(val);
				println!("url : {}, value : {}", url, value);
				return value;
			}
		}
	}
	"Not Found".to_string()
}

fn get_block(index : i32, num_block : i32) -> Option<Block> {
	// Url
	let mut url :String = "http://etherscan.io/block/".to_owned();
	let current_block = num_block.to_string();
	// Create new url
	url.push_str(&current_block);
	// Get the doc
	let doc_text = get_text(&url);

	// Check if doc_text is none
	if doc_text == "Nothing" {
		return None;
	}

	// Parse the doc
	let doc = Html::parse_document(&doc_text);
	// Get selector
	let selector_class = Selector::parse(r#"table[id="ContentPlaceHolder1_maintable"]"#).unwrap();
	let selector_tr = Selector::parse("tr").unwrap();
	let selector_td = Selector::parse("td").unwrap();
	// Get the html
	let element_class = doc.select(&selector_class).next().unwrap();
	// Get the td
	let element_tr = element_class.select(&selector_tr);
	// New Vector
	let mut res : Vec<String> = Vec::new();
	// Iterate the elements
	for el in element_tr {
		// Get element td
		let mut element_td = el.select(&selector_td);
		// Get left column
		let left_col = element_td.next().unwrap().text().collect::<Vec<_>>();
		// Get right column
		let right_col = element_td.next().unwrap().text().collect::<Vec<_>>();
		// Create vector of string 
		let mut left_res = String::new();
		let mut right_res = String::new();
		// Get left string
		let left_string = (left_col.get(0).unwrap()).replace("\n", "").replace("\u{a0}", "").replace(":", "");
		left_res.push_str(&left_string);
		// Get the data!!
		if left_string == "Transactions" {
			// Well, there's two results for transactions.. actually tree
			if right_col.len() == 3 {
				res.push(right_col[1].to_string());
				let str_internal = right_col.get(2).unwrap().to_string().replace("and", "").replace("in this block", "");
				let str_internal = str_internal.trim();
				res.push(str_internal.to_string());
			} else if right_col.len() == 1 {
				res.push("0 transactions".to_string());
				res.push("0 contract internal transactions".to_string());
			} else {
				res.push(right_col[1].to_string());
				res.push(right_col[3].to_string());
			}
			continue;
		}

		for right in right_col {
			if left_string == "Height" {
				right_res.push_str(&num_block.to_string());
				break;
			}
			let right_string = right.replace("\n", "").replace("\u{a0}", "");
			right_res.push_str(&right_string);
		}
		res.push(right_res);		
	}
	println!("{}. Success Getting Block {}",index, num_block);
	//println!("{:?}", res);
	// Return
	Some(Block {
		height : res.get(0).unwrap().to_string(),
		time_stamp : res.get(1).unwrap().to_string(),
		transactions : res.get(2).unwrap().to_string(),
		internal_transactions : res.get(3).unwrap().to_string(),
		hash : res.get(4).unwrap().to_string(),
		parent_hash : res.get(5).unwrap().to_string(),
		sha3uncles : res.get(6).unwrap().to_string(),
		mined_by : res.get(7).unwrap().to_string(),
		difficulty : res.get(8).unwrap().to_string(),
		total_difficulty : res.get(9).unwrap().to_string(),
		size : res.get(10).unwrap().to_string(),
		gas_used : res.get(11).unwrap().to_string(),
		gas_limit : res.get(12).unwrap().to_string(),
		nonce : res.get(13).unwrap().to_string(),
		block_reward : res.get(14).unwrap().to_string(),
		uncles_reward : res.get(15).unwrap().to_string(),
	})
}