--  Fucking Shit rcp1 was a shithole to make and i cant code for shit in lua so this is the best it is gents
return {
    rcp1 = {
        port = 8080,
        max_connections = 100,
        connection_timeout = 30000,
        log_level = "INFO"
    },
    security = {
        enable_encryption = true,
        key_file = "/etc/zamarine/rcp1/keys/rcp1.key"
    },
    performance = {
        thread_pool_size = 10,
        buffer_size = 8192,
        max_message_size = 1048576
    }
}
