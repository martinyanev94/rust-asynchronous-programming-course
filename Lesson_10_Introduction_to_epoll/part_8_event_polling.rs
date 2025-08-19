let mut events = vec![ffi::Event { events: 0, epoll_data: 0 }; 10];
loop {
    poll.poll(&mut events, None)?;
    handle_events(&events, &mut streams)?;
}
