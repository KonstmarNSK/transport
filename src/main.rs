use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

fn main() {
    println!("Hello, world!");
}




fn process_producer_message(msg: ProducerMessage){

}

fn find_partition_for_message(state: &State, msg: ProducerMessage) -> Option<Partition>{
    let maybe_topic = state.topics.iter().find(|item| { item.name.eq(&msg.topic) });

    maybe_topic.and_then(|topic| {topic.partitions.first()}).cloned()
}

fn get_msg_hash(msg: ProducerMessage) -> u64{
    let mut hasher = DefaultHasher::new();
    std::hash::Hash::hash(&msg.topic, &mut hasher);
    hasher.finish()
}

impl Partition {
    fn accept_new_message(&mut self, msg: &ProducerMessage){

    }
}

struct State{
    topics: Vec<Topic>
}

struct Topic{
    name: String,
    partitions: Vec<Partition>
}


struct ProducerMessage<'a>{
    topic: String,
    content: &'a [u8]
}

#[derive(Copy, Clone)]
struct Partition;
struct Producer;
struct Consumer;
struct Broker;


