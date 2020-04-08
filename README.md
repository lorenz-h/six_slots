# six_slots
six_slots is a slot parser for [Six](https://github.com/lorenz-h/six). It recieves a task json-string consisting of a uid, a user utterance and the type of slots to search for, and returns result json-string consisting of the tasks uid and a list of found values in the utterance.  
Numerical Slots are parsed using the [Snips Rustling crate](https://github.com/snipsco/rustling-ontology).
