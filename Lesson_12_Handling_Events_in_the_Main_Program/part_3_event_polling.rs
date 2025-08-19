let mut events = vec![Event::default(); 128]; // Allocate space for events
loop {
    ...
    let num_events = registry.poll.poll(&mut events, Some(100)).unwrap();
    for i in 0..num_events {
        let event = &events[i];
        handle_events(event, &incoming_streams).unwrap();
    }
}
