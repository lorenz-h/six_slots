use std::str::FromStr;
use rumqtt::{MqttClient, MqttOptions, QoS, Notification};

use serde::{Serialize, Deserialize};
use rustling_ontology::*;
use rustling_ontology_json_utils::SlotValue;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    uid: i32,
    statement: String,
    slot_kinds: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    uid: i32,
    found_slots: Vec<String>,
}

fn parse_task(json_str: &str) -> serde_json::Result<Task> {    
    let task: Task = serde_json::from_str(&json_str)?;
    println!("New Task: Trying to parse Slots {:?} from <{}>.", task.slot_kinds, task.statement);
    Ok(task)
}

fn fill_slots(context: &ResolverContext, parser: &rustling_ontology::Parser, task: &Task) -> Vec<String>{
    let statement = task.statement.to_lowercase();

    let slot_kinds = task.slot_kinds.iter().map(|value| OutputKind::from_str(&value).unwrap()).collect::<Vec<_>>();    
    let entities = parser.parse_with_kind_order(&*statement, &context, &slot_kinds).unwrap();
    if entities.len() != 0 {

        let found_slots: Vec<String> = entities.iter().rev().map(|c| {
            let slot_value: SlotValue = SlotValue::from(c.value.clone());
            let serialized: String = serde_json::to_string(&slot_value).unwrap();
            return serialized;
        }).collect();

        return found_slots;

    } else {
        println!("Did not find any entities in statement {}",statement );
        return Vec::new();
    } 
  
}

fn generate_response(task: &Task, found_slots: &Vec<String>) -> std::string::String{
    let response = Response {
        uid: task.uid.clone(),
        found_slots: found_slots.clone(),
    };
    println!("Generated response: {:?}", response);
    return serde_json::to_string(&response).unwrap();
}


fn main() {
    // test:  mosquitto_pub -h "192.168.52.4" -t "six/slots/tasks" -m '{\"statement\": \"Set the temperature in the kitchen to 23 degrees tomorrow\", \"slot_kinds\": [\"Temperature\",\"Date\"], \"uid\":121938}'
    let task_topic = String::from("six/slots/tasks");

    let mqtt_options = MqttOptions::new("six_slots", "localhost", 1883);
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
      
    mqtt_client.subscribe(&task_topic, QoS::AtLeastOnce).unwrap();

    let context = ResolverContext::default();
    let parser = build_parser(rustling_ontology::Lang::EN).unwrap();

    for notification in notifications {
        match notification{
            Notification::Publish(ref msg) => {
                if msg.topic_name == task_topic{
                    let decoded_payload = std::str::from_utf8(&msg.payload).unwrap();

                    let task = parse_task(decoded_payload);

                    match task {
                        Ok(task) => {
                            let found_slots = fill_slots(&context, &parser, &task);
                            let response = generate_response(&task, &found_slots);
                            mqtt_client.publish("six/slots/results", QoS::AtLeastOnce, false, response.into_bytes()).unwrap();
                        },
                        Err(e) => println!("error parsing payload {} as json: {:?}",decoded_payload, e),
                    };
                }
                else{
                    println!("Recieved non task message {}", std::str::from_utf8(&msg.payload).unwrap())
                }
            },
            _ => println!("recieved other notification {:?}", notification ),
        }
    }
}
