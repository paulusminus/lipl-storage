pub fn exit_on_signal_int() {
    tracing::info!("Exiting because of signal INT");
}

pub fn exit_on_signal_term() {
    tracing::info!("Exiting because of signal TERM");
}
