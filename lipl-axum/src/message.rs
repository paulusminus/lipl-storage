pub fn exit_on_signal_int() {
    tracing::info!("Exiting because of signal INT");
}

pub fn error_on_receiving_signal(error: std::io::Error) {
    tracing::error!("Error receiving signal: {}", error);
}
